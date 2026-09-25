#[cfg(feature = "legacy-uploads")]
use aws_sdk_s3::Client as S3Client;
use axum::{http, routing::get, Router};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};

mod config;
mod db;
mod error;
mod middleware;
mod models;
mod pilot;
mod routes;
mod services;
#[cfg(test)]
mod tests;

use config::Config;
use services::email::EmailService;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    #[cfg(feature = "legacy-uploads")]
    pub s3: S3Client,
    pub email: EmailService,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // inicializar tracing con formato estructurado
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tracing::info!("Iniciando servidor API B2B Product Catalog Quote System v0.2.0");

    // cargar configuracion
    let config =
        Config::from_env().map_err(|e| anyhow::anyhow!("Error al cargar configuracion: {}", e))?;

    tracing::info!("Configuracion cargada exitosamente");

    // inicializar pool de base de datos
    let db_pool = db::create_pool(&config.database_url).await?;
    tracing::info!("Pool de conexiones de base de datos creado");

    // Ejecutar migraciones
    tracing::info!("Ejecutando migraciones de base de datos...");
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    tracing::info!("Migraciones de base de datos completadas");
    if std::env::args().any(|a| a == "bootstrap-admin") {
        let email = std::env::var("BOOTSTRAP_ADMIN_EMAIL")?;
        let password = std::env::var("BOOTSTRAP_ADMIN_PASSWORD")?;
        anyhow::ensure!(
            email.contains('@') && password.len() >= 16,
            "Use valid email and a unique password of at least 16 characters"
        );
        let hash = services::auth::hash_password(&password)?;
        let mut tx = db_pool.begin().await?;
        sqlx::query("LOCK TABLE admins IN EXCLUSIVE MODE")
            .execute(&mut *tx)
            .await?;
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM admins")
            .fetch_one(&mut *tx)
            .await?;
        anyhow::ensure!(
            count.0 == 0,
            "Bootstrap refused: administrator already exists"
        );
        sqlx::query(
            "INSERT INTO admins (email,password_hash,name) VALUES ($1,$2,'Responsable del piloto')",
        )
        .bind(email)
        .bind(hash)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        tracing::info!(
            "Administrator created. Remove BOOTSTRAP_ADMIN_PASSWORD before starting the server."
        );
        return Ok(());
    }

    // inicializar servicios
    #[cfg(feature = "legacy-uploads")]
    let s3_client = services::s3::create_client(&config).await;
    #[cfg(feature = "legacy-uploads")]
    tracing::info!("Cliente S3 inicializado");

    let email_service = EmailService::new(&config);
    tracing::info!("Servicio de email inicializado");

    // Guardar puerto antes de mover config
    let port = config.port;
    let cors_origins = config.cors_origin.clone();

    let app_state = AppState {
        db: db_pool,
        #[cfg(feature = "legacy-uploads")]
        s3: s3_client,
        email: email_service,
        config,
    };

    // configurar cors estricto - sin AllowAll
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(
            cors_origins
                .iter()
                .map(|origin| origin.parse().unwrap())
                .collect::<Vec<_>>(),
        ))
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::PATCH,
            http::Method::DELETE,
        ])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
        ]);

    let app = app(app_state).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Servidor escuchando en {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

pub fn app(app_state: AppState) -> Router {
    let public_routes = if app_state.config.legacy_public_api {
        routes::public::routes()
    } else {
        Router::new()
    };
    Router::new()
        .route("/health", get(health_check))
        .nest("/api", public_routes)
        .nest("/api/admin", routes::admin::routes(app_state.clone()))
        .nest("/api/pilot", pilot::routes(app_state.clone()))
        .route(
            "/pilot",
            get(|| async { axum::response::Html(include_str!("../static/pilot.html")) }),
        )
        .layer(axum::extract::DefaultBodyLimit::max(64 * 1024))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(app_state)
}

async fn health_check() -> &'static str {
    "OK"
}
