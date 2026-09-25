use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_region: String,
    pub aws_s3_bucket: String,
    pub email_api_key: String,
    pub email_from: String,
    pub email_to: String,
    pub cors_origin: Vec<String>,
    pub legacy_public_api: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenv().ok();

        let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET must be set")?;
        if jwt_secret.len() < 32 || jwt_secret.contains("your-secret") || jwt_secret.to_uppercase().contains("REPLACE") {
            return Err("JWT_SECRET must be a random secret of at least 32 characters".into());
        }
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set".to_string())?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| "PORT must be a valid number".to_string())?,
            jwt_secret,
            legacy_public_api: env::var("ENABLE_LEGACY_PUBLIC_API").as_deref() == Ok("true"),
            aws_access_key_id: env::var("AWS_ACCESS_KEY_ID").unwrap_or_default(),
            aws_secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_default(),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            aws_s3_bucket: env::var("AWS_S3_BUCKET").unwrap_or_default(),
            email_api_key: env::var("EMAIL_API_KEY").unwrap_or_default(),
            email_from: env::var("EMAIL_FROM").unwrap_or_default(),
            email_to: env::var("EMAIL_TO").unwrap_or_default(),
            cors_origin: env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:4321".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        })
    }
}
