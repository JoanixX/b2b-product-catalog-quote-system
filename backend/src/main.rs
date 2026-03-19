use axum::{Router, routing::get, http};
use tower_http::cors::{CorsLayer, AllowOrigin};
use std::net::SocketAddr;
use sqlx::PgPool;
use aws_sdk_s3::Client as S3Client;

mod config;
mod db;
mod error;
mod models;
mod routes;
mod services;
mod middleware;

use config::Config;
use services::email::EmailService;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
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

    tracing::info!("Iniciando servidor API LabMedical v0.2.0");

    // cargar configuracion
    let config = Config::from_env()
        .map_err(|e| anyhow::anyhow!("Error al cargar configuracion: {}", e))?;
    
    tracing::info!("Configuracion cargada exitosamente");
    
    // inicializar pool de base de datos
    let db_pool = db::create_pool(&config.database_url).await?;
    tracing::info!("Pool de conexiones de base de datos creado");
    
    // Ejecutar migraciones
    tracing::info!("Ejecutando migraciones de base de datos...");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await?;
    tracing::info!("Migraciones de base de datos completadas");
    
    // inicializar servicios
    let s3_client = services::s3::create_client(&config).await;
    tracing::info!("Cliente S3 inicializado");
    
    let email_service = EmailService::new(&config);
    tracing::info!("Servicio de email inicializado");
    
    // Guardar puerto antes de mover config
    let port = config.port;
    let cors_origins = config.cors_origin.clone();
    
    let app_state = AppState {
        db: db_pool,
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
                .collect::<Vec<_>>()
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
    
    // construir router
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", routes::public::routes())
        .nest("/api/admin", routes::admin::routes())
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(app_state);
    
    // Iniciar servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Servidor escuchando en {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}