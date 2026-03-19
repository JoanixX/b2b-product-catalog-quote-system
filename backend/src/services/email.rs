use crate::{config::Config, error::{ApiError, ApiResult}};

#[derive(Clone)]
pub struct EmailService {
    from: String,
    to: String,
    api_key: String,
}

impl EmailService {
    pub fn new(config: &Config) -> Self {
        Self {
            from: config.email_from.clone(),
            to: config.email_to.clone(),
            api_key: config.email_api_key.clone(),
        }
    }
    
    /// envia notificacion de cotizacion con plantilla html profesional
    pub async fn send_quote_notification(
        &self,
        company_name: &str,
        contact_name: &str,
        email: &str,
        phone: Option<&str>,
        ruc: &str,
        products: &str,
        message: Option<&str>,
    ) -> ApiResult<()> {
        let phone_display = phone.unwrap_or("No proporcionado");
        let message_display = message.unwrap_or("Ninguno");
        
        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <style>
        body {{ font-family: 'Segoe UI', Arial, sans-serif; margin: 0; padding: 0; background: #f5f5f5; }}
        .container {{ max-width: 600px; margin: 20px auto; background: white; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }}
        .header {{ background: linear-gradient(135deg, #1e40af 0%, #3b82f6 100%); color: white; padding: 24px 32px; }}
        .header h1 {{ margin: 0; font-size: 20px; font-weight: 600; }}
        .header p {{ margin: 4px 0 0; opacity: 0.9; font-size: 14px; }}
        .body {{ padding: 32px; }}
        .section {{ margin-bottom: 24px; }}
        .section h2 {{ font-size: 14px; color: #6b7280; text-transform: uppercase; letter-spacing: 1px; margin: 0 0 12px; }}
        table {{ width: 100%; border-collapse: collapse; }}
        table td {{ padding: 8px 0; border-bottom: 1px solid #f3f4f6; }}
        table td:first-child {{ color: #6b7280; width: 140px; font-size: 14px; }}
        table td:last-child {{ color: #1f2937; font-weight: 500; }}
        .products {{ background: #f9fafb; border-radius: 6px; padding: 16px; margin-top: 8px; }}
        .message {{ background: #fffbeb; border-left: 3px solid #f59e0b; padding: 12px 16px; border-radius: 0 6px 6px 0; }}
        .footer {{ background: #f9fafb; padding: 16px 32px; text-align: center; font-size: 12px; color: #9ca3af; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Nueva Solicitud de Cotizacion</h1>
            <p>LabMedical - Sistema de Cotizaciones</p>
        </div>
        <div class="body">
            <div class="section">
                <h2>Datos de la Empresa</h2>
                <table>
                    <tr><td>Razon Social</td><td>{company_name}</td></tr>
                    <tr><td>RUC</td><td>{ruc}</td></tr>
                </table>
            </div>
            <div class="section">
                <h2>Datos de Contacto</h2>
                <table>
                    <tr><td>Nombre</td><td>{contact_name}</td></tr>
                    <tr><td>Email</td><td>{email}</td></tr>
                    <tr><td>Telefono</td><td>{phone_display}</td></tr>
                </table>
            </div>
            <div class="section">
                <h2>Productos Solicitados</h2>
                <div class="products">{products}</div>
            </div>
            <div class="section">
                <h2>Mensaje Adicional</h2>
                <div class="message">{message_display}</div>
            </div>
        </div>
        <div class="footer">
            Este es un mensaje automatico del sistema de cotizaciones de LabMedical.
        </div>
    </div>
</body>
</html>"#
        );
        
        self.send_email(
            &format!("Cotización - {} (RUC: {})", company_name, ruc),
            &html_body,
            true,
        ).await
    }
    
    async fn send_email(&self, subject: &str, body: &str, is_html: bool) -> ApiResult<()> {
        let client = reqwest::Client::new();
        
        let mut payload = serde_json::json!({
            "from": self.from,
            "to": [self.to],
            "subject": subject,
        });
        
        if is_html {
            payload["html"] = serde_json::Value::String(body.to_string());
        } else {
            payload["text"] = serde_json::Value::String(body.to_string());
        }
        
        let response = client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(
                    error_type = "email_send",
                    subject = subject,
                    "Error al enviar email: {}", e
                );
                ApiError::Internal("Error al enviar notificación".to_string())
            })?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Desconocido".to_string());
            tracing::error!(
                error_type = "email_api",
                status = %status,
                details = %error_text,
                "Error del servicio de email"
            );
            return Err(ApiError::Internal("Error al enviar notificación".to_string()));
        }
        
        tracing::info!(subject = subject, "Email enviado exitosamente");
        Ok(())
    }
}