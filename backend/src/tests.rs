use super::*;
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use tower::ServiceExt;

async fn call(
    app: &Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Value,
) -> (StatusCode, Value) {
    let mut req = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json");
    if let Some(t) = token {
        req = req.header("authorization", format!("Bearer {t}"));
    }
    let response = app
        .clone()
        .oneshot(req.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1_000_000).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[sqlx::test(migrations = "./migrations")]
async fn security_and_pilot_vertical(pool: PgPool) {
    let config = Config {
        database_url: String::new(),
        port: 3000,
        jwt_secret: uuid::Uuid::new_v4().to_string(),
        aws_access_key_id: String::new(),
        aws_secret_access_key: String::new(),
        aws_region: "us-east-1".into(),
        aws_s3_bucket: String::new(),
        email_api_key: String::new(),
        email_from: String::new(),
        email_to: String::new(),
        cors_origin: vec![],
        legacy_public_api: true,
    };
    let state = AppState {
        db: pool.clone(),
        email: EmailService::new(&config),
        config: config.clone(),
        #[cfg(feature = "legacy-uploads")]
        s3: services::s3::create_client(&config).await,
    };
    let router = app(state);
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM admins")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        count.0, 0,
        "Historical seeded access must be removed before serving"
    );
    let protected = [
        ("GET", "/api/admin/products"),
        ("POST", "/api/admin/products"),
        ("PUT", "/api/admin/products/1"),
        ("DELETE", "/api/admin/products/1"),
        ("PATCH", "/api/admin/products/1/toggle"),
        ("GET", "/api/admin/categories"),
        ("POST", "/api/admin/categories"),
        ("PUT", "/api/admin/categories/1"),
        ("DELETE", "/api/admin/categories/1"),
        ("GET", "/api/admin/quotes"),
        ("GET", "/api/admin/quotes/1"),
        ("PATCH", "/api/admin/quotes/1/status"),
        ("POST", "/api/admin/upload"),
        ("GET", "/api/pilot/quotes"),
        ("POST", "/api/pilot/quotes"),
        (
            "GET",
            "/api/pilot/quotes/00000000-0000-0000-0000-000000000000",
        ),
        (
            "POST",
            "/api/pilot/quotes/00000000-0000-0000-0000-000000000000/action",
        ),
    ];
    for (method, path) in protected {
        for token in [None, Some("invalid.token.value")] {
            assert_eq!(
                call(&router, method, path, token, json!({})).await.0,
                StatusCode::UNAUTHORIZED,
                "{method} {path}"
            );
        }
    }
    let email = "operator@example.invalid";
    let password = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO admins(email,password_hash) VALUES($1,$2)")
        .bind(email)
        .bind(services::auth::hash_password(&password).unwrap())
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        call(
            &router,
            "POST",
            "/api/admin/login",
            None,
            json!({"email":email,"password":"incorrect-password"})
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let (status, login) = call(
        &router,
        "POST",
        "/api/admin/login",
        None,
        json!({"email":email,"password":password}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let token = login["token"].as_str().unwrap();
    let revoked =
        services::auth::generate_jwt("revoked@example.invalid", &config.jwt_secret).unwrap();
    assert_eq!(
        call(
            &router,
            "GET",
            "/api/pilot/quotes",
            Some(&revoked),
            json!({})
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );

    sqlx::query("INSERT INTO products(name,slug,brand,registro_sanitario) VALUES('Synthetic item','synthetic-item','Example','example-only')").execute(&pool).await.unwrap();
    let (_, all) = call(&router, "GET", "/api/products", None, json!({})).await;
    assert_eq!(all["total"], 1);
    for path in [
        "/api/products?search=%27%20OR%201%3D1--",
        "/api/products?category=%27%29%20OR%201%3D1--",
    ] {
        let (status, data) = call(&router, "GET", path, None, json!({})).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(data["total"], 0);
        assert_eq!(data["products"], json!([]));
    }
    let (status, data) = call(
        &router,
        "GET",
        "/api/admin/quotes?status=%27%20OR%201%3D1--",
        Some(token),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(data["total"], 0);

    let id = uuid::Uuid::new_v4();
    let request = json!({"id":id,"company":"Synthetic buyer","contact":"Example person","product":"Example machine","quantity":2});
    let (status, created) = call(
        &router,
        "POST",
        "/api/pilot/quotes",
        Some(token),
        request.clone(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(created["saved"], true);
    assert_eq!(created["notification_status"], "disabled");
    let (_, replay) = call(
        &router,
        "POST",
        "/api/pilot/quotes",
        Some(token),
        request.clone(),
    )
    .await;
    assert_eq!(replay["replayed"], true);
    let mut changed = request;
    changed["quantity"] = json!(3);
    assert_eq!(
        call(&router, "POST", "/api/pilot/quotes", Some(token), changed)
            .await
            .0,
        StatusCode::CONFLICT
    );
    let path = format!("/api/pilot/quotes/{id}/action");
    assert_eq!(
        call(
            &router,
            "POST",
            &path,
            Some(token),
            json!({"version":1,"action":"proposal"})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        call(
            &router,
            "POST",
            &path,
            Some(token),
            json!({"version":1,"action":"draft"})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        call(
            &router,
            "POST",
            &path,
            Some(token),
            json!({"version":2,"action":"approve","human_confirmed":true})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let edit = json!({"version":2,"action":"edit","quantity":2,"unit_price_cents":10001,"tax_bps":1800,"currency":"PEN","terms":"Synthetic terms, human review required"});
    let mut invalid = edit.clone();
    invalid["unit_price_cents"] = json!(-1);
    assert_eq!(
        call(&router, "POST", &path, Some(token), invalid).await.0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        call(&router, "POST", &path, Some(token), edit.clone())
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        call(&router, "POST", &path, Some(token), edit).await.0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        call(
            &router,
            "POST",
            &path,
            Some(token),
            json!({"version":3,"action":"approve"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        call(
            &router,
            "POST",
            &path,
            Some(token),
            json!({"version":3,"action":"approve","human_confirmed":true})
        )
        .await
        .0,
        StatusCode::OK
    );
    let (status, proposal) = call(
        &router,
        "POST",
        &path,
        Some(token),
        json!({"version":4,"action":"proposal"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(proposal["proposal"]["total_cents"], 23602);
    assert_eq!(proposal["approved_by"], email);
    assert_eq!(
        call(
            &router,
            "POST",
            &path,
            Some(token),
            json!({"version":5,"action":"edit"})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        call(
            &router,
            "POST",
            &path,
            Some(token),
            json!({"version":5,"action":"followup"})
        )
        .await
        .0,
        StatusCode::OK
    );
    let (_, detail) = call(
        &router,
        "GET",
        &format!("/api/pilot/quotes/{id}"),
        Some(token),
        json!({}),
    )
    .await;
    assert_eq!(detail["quote"]["status"], "followup");
    assert_eq!(detail["events"].as_array().unwrap().len(), 6);
    assert_eq!(
        detail["quote"]["proposal"], proposal["proposal"],
        "Proposal remains immutable"
    );
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pilot_quotes")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 1);
}
