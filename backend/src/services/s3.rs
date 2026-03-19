use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, primitives::ByteStream};
use crate::{config::Config, error::{ApiError, ApiResult}};
use uuid::Uuid;

pub async fn create_client(config: &Config) -> Client {
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(aws_config::Region::new(config.aws_region.clone()))
        .load()
        .await;
    
    Client::new(&aws_config)
}

// tipos permitidos para subida de archivos
const ALLOWED_IMAGE_TYPES: &[&str] = &["image/jpeg", "image/webp"];
const ALLOWED_DOC_TYPES: &[&str] = &["application/pdf"];

// se sube un archivo validando el mime-type estrictamente
// solo se permiten: jpeg, webp (imagenes) y pdf (fichas tecnicas)
// los nombres se generan como uuid para evitar colisiones y enumeracion
pub async fn upload_file(
    client: &Client,
    bucket: &str,
    file_data: Vec<u8>,
    content_type: &str,
) -> ApiResult<String> {
    // validar mime-type estrictamente
    let (extension, folder) = if ALLOWED_IMAGE_TYPES.contains(&content_type) {
        let ext = match content_type {
            "image/jpeg" => "jpg",
            "image/webp" => "webp",
            _ => return Err(ApiError::BadRequest("Tipo de imagen no permitido".to_string())),
        };
        (ext, "images")
    } else if ALLOWED_DOC_TYPES.contains(&content_type) {
        ("pdf", "documents")
    } else {
        tracing::warn!(
            content_type = content_type,
            "Intento de subida con tipo mime no permitido"
        );
        return Err(ApiError::BadRequest(
            "Tipo de archivo no permitido. Solo se aceptan: JPEG, WebP y PDF".to_string()
        ));
    };
    
    // generar nombre uuid para evitar colisiones y ataques de enumeracion
    let filename = format!("{}.{}", Uuid::new_v4(), extension);
    let key = format!("products/{}/{}", folder, filename);
    
    // Subir a s3
    client
        .put_object()
        .bucket(bucket)
        .key(&key)
        .body(ByteStream::from(file_data))
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| {
            tracing::error!(
                error_type = "s3_upload",
                bucket = bucket,
                key = key,
                "Error al subir archivo a s3: {}", e
            );
            ApiError::Internal("Error al subir archivo".to_string())
        })?;
    
    let url = format!("https://{}.s3.amazonaws.com/{}", bucket, key);
    tracing::info!(url = url, content_type = content_type, "Archivo subido exitosamente");
    Ok(url)
}

#[allow(dead_code)]
pub async fn delete_image(
    client: &Client,
    bucket: &str,
    url: &str,
) -> ApiResult<()> {
    // se extrae la key de la URL
    let key = url
        .split(&format!("{}.s3.amazonaws.com/", bucket))
        .nth(1)
        .ok_or_else(|| ApiError::BadRequest("URL de S3 inválida".to_string()))?;
    
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| {
            tracing::error!(
                error_type = "s3_delete",
                bucket = bucket,
                key = key,
                "Error al eliminar archivo de s3: {}", e
            );
            ApiError::Internal("Error al eliminar archivo".to_string())
        })?;
    
    Ok(())
}