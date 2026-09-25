use crate::{
    error::{ApiError, ApiResult},
    services::auth::Claims,
    AppState,
};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::FromRow;
use uuid::Uuid;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/quotes", get(list).post(create))
        .route("/quotes/:id", get(detail))
        .route("/quotes/:id/action", post(action))
        .route_layer(axum::middleware::from_fn_with_state(
            state,
            crate::middleware::auth::auth_middleware,
        ))
}

#[derive(Debug, Serialize, FromRow)]
pub struct Quote {
    id: Uuid,
    request: Value,
    company: String,
    contact: String,
    product: String,
    quantity: i32,
    unit_price_cents: i64,
    tax_bps: i32,
    currency: String,
    terms: String,
    status: String,
    version: i32,
    approved_by: Option<String>,
    approved_at: Option<chrono::DateTime<chrono::Utc>>,
    proposal: Option<Value>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Serialize)]
struct Request {
    id: Uuid,
    company: String,
    contact: String,
    product: String,
    quantity: i32,
    #[serde(default)]
    notes: String,
}

fn invalid(message: &str) -> ApiError {
    ApiError::BadRequest(message.into())
}
fn text_valid(value: &str, max: usize) -> bool {
    !value.trim().is_empty() && value.chars().count() <= max
}

async fn create(
    State(s): State<AppState>,
    Extension(user): Extension<Claims>,
    Json(p): Json<Request>,
) -> ApiResult<Json<Value>> {
    if !text_valid(&p.company, 200)
        || !text_valid(&p.contact, 200)
        || !text_valid(&p.product, 200)
        || !(1..=10000).contains(&p.quantity)
        || p.notes.chars().count() > 2000
    {
        return Err(invalid(
            "Revisa empresa, contacto, producto y cantidad (1–10000).",
        ));
    }
    let request = serde_json::to_value(&p).map_err(|_| invalid("Solicitud inválida"))?;
    let mut tx = s.db.begin().await?;
    let inserted = sqlx::query("INSERT INTO pilot_quotes (id, request, company, contact, product, quantity) VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT (id) DO NOTHING")
        .bind(p.id).bind(&request).bind(&p.company).bind(&p.contact).bind(&p.product).bind(p.quantity).execute(&mut *tx).await?.rows_affected() == 1;
    let q: Quote = sqlx::query_as("SELECT * FROM pilot_quotes WHERE id=$1")
        .bind(p.id)
        .fetch_one(&mut *tx)
        .await?;
    if q.request != request {
        return Err(ApiError::Conflict(
            "Identificador reutilizado para otro pedido".into(),
        ));
    }
    if inserted {
        sqlx::query(
            "INSERT INTO pilot_events (quote_id,actor,action,version) VALUES ($1,$2,'received',1)",
        )
        .bind(p.id)
        .bind(&user.sub)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(Json(
        json!({"quote": q, "saved": true, "replayed": !inserted, "notification_status": "disabled"}),
    ))
}

async fn list(State(s): State<AppState>) -> ApiResult<Json<Vec<Quote>>> {
    Ok(Json(
        sqlx::query_as("SELECT * FROM pilot_quotes ORDER BY created_at DESC LIMIT 100")
            .fetch_all(&s.db)
            .await?,
    ))
}

async fn detail(State(s): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<Value>> {
    let q: Quote = sqlx::query_as("SELECT * FROM pilot_quotes WHERE id=$1")
        .bind(id)
        .fetch_optional(&s.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("Solicitud".into()))?;
    let events: Vec<(String, String, i32, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT actor,action,version,created_at FROM pilot_events WHERE quote_id=$1 ORDER BY id",
    )
    .bind(id)
    .fetch_all(&s.db)
    .await?;
    Ok(Json(json!({"quote":q,"events":events})))
}

#[derive(Deserialize)]
struct Action {
    version: i32,
    action: String,
    quantity: Option<i32>,
    unit_price_cents: Option<i64>,
    tax_bps: Option<i32>,
    currency: Option<String>,
    terms: Option<String>,
    #[serde(default)]
    human_confirmed: bool,
}

async fn action(
    State(s): State<AppState>,
    Extension(user): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(p): Json<Action>,
) -> ApiResult<Json<Quote>> {
    let mut tx = s.db.begin().await?;
    let mut q: Quote = sqlx::query_as("SELECT * FROM pilot_quotes WHERE id=$1 FOR UPDATE")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| ApiError::NotFound("Solicitud".into()))?;
    if q.version != p.version {
        return Err(ApiError::Conflict(
            "Otro usuario modificó el pedido. Recarga antes de continuar.".into(),
        ));
    }
    match (q.status.as_str(), p.action.as_str()) {
        ("received", "draft") => q.status = "draft".into(),
        ("draft", "edit") => {
            let quantity = p.quantity.ok_or_else(|| invalid("Falta cantidad"))?;
            let price = p
                .unit_price_cents
                .ok_or_else(|| invalid("Falta precio en céntimos"))?;
            let tax = p
                .tax_bps
                .ok_or_else(|| invalid("Falta impuesto en puntos básicos"))?;
            let currency = p.currency.ok_or_else(|| invalid("Falta moneda"))?;
            let terms = p.terms.ok_or_else(|| invalid("Faltan condiciones"))?;
            if !(1..=10000).contains(&quantity)
                || !(0..=1_000_000_000).contains(&price)
                || !(0..=10000).contains(&tax)
                || !["PEN", "USD"].contains(&currency.as_str())
                || !text_valid(&terms, 2000)
            {
                return Err(invalid(
                    "Importes, cantidad, moneda o condiciones inválidos",
                ));
            }
            q.quantity = quantity;
            q.unit_price_cents = price;
            q.tax_bps = tax;
            q.currency = currency;
            q.terms = terms;
        }
        ("draft", "approve") => {
            if !p.human_confirmed || q.unit_price_cents <= 0 || q.terms.trim().is_empty() {
                return Err(invalid("Revisa precio positivo, impuestos y condiciones; confirma la aprobación humana."));
            }
            q.status = "approved".into();
            q.approved_by = Some(user.sub.clone());
            q.approved_at = Some(chrono::Utc::now());
        }
        ("approved", "proposal") => {
            let subtotal = q.unit_price_cents * i64::from(q.quantity);
            let tax = (subtotal * i64::from(q.tax_bps) + 5000) / 10000;
            q.proposal = Some(
                json!({"id":q.id,"company":q.company,"contact":q.contact,"product":q.product,
                "quantity":q.quantity,"unit_price_cents":q.unit_price_cents,"subtotal_cents":subtotal,
                "tax_bps":q.tax_bps,"tax_cents":tax,"total_cents":subtotal+tax,"currency":q.currency,"terms":q.terms,
                "approved_by":q.approved_by,"approved_at":q.approved_at,"label":"PILOTO SINTÉTICO · NO ENVIADO"}),
            );
            q.status = "proposal".into();
        }
        ("proposal", "followup") => q.status = "followup".into(),
        _ => {
            return Err(ApiError::Conflict(
                "Transición no permitida. Revisa el estado actual.".into(),
            ))
        }
    }
    let updated: Quote = sqlx::query_as("UPDATE pilot_quotes SET quantity=$2, unit_price_cents=$3, tax_bps=$4, currency=$5, terms=$6, status=$7, approved_by=$8, approved_at=$9, proposal=$10, version=version+1, updated_at=NOW() WHERE id=$1 RETURNING *")
        .bind(id).bind(q.quantity).bind(q.unit_price_cents).bind(q.tax_bps).bind(q.currency).bind(q.terms).bind(q.status)
        .bind(q.approved_by).bind(q.approved_at).bind(q.proposal).fetch_one(&mut *tx).await?;
    sqlx::query("INSERT INTO pilot_events (quote_id,actor,action,version) VALUES ($1,$2,$3,$4)")
        .bind(id)
        .bind(user.sub)
        .bind(p.action)
        .bind(updated.version)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(Json(updated))
}
