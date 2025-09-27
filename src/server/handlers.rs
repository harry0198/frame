use axum::http::StatusCode;
use axum::extract::Multipart;
use axum::response::Json;
use serde_json::{json, Value};
use crate::file_store::{self, FileStoreError};

impl From<FileStoreError> for StatusCode {
    fn from(error: FileStoreError) -> Self {
        match error {
            FileStoreError::InvalidFileType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            FileStoreError::FileTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            FileStoreError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            FileStoreError::InvalidPath => StatusCode::BAD_REQUEST,
        }
    }
}

pub async fn get_images() -> Result<Json<Value>, StatusCode> {
    match file_store::list_uploaded_files().await {
        Ok(images) => Ok(Json(json!({
            "images": images,
            "count": images.len()
        }))),
        Err(e) => {
            eprintln!("Error listing images: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn upload_image(mut multipart: Multipart) -> Result<Json<Value>, StatusCode> {
    // Ensure upload directory exists
    file_store::ensure_upload_directory().await.map_err(|e| {
        eprintln!("Failed to create upload directory: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut uploaded_files = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        eprintln!("Multipart error: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        let field_name = field.name().ok_or_else(|| {
            eprintln!("Field without name");
            StatusCode::BAD_REQUEST
        })?.to_string();

        let filename = field.file_name().ok_or_else(|| {
            eprintln!("Field without filename");
            StatusCode::BAD_REQUEST
        })?.to_string();

        // Read file data
        let data = field.bytes().await.map_err(|e| {
            eprintln!("Failed to read field data: {}", e);
            StatusCode::BAD_REQUEST
        })?;

        // Save file using file_store module
        match file_store::save_file(&filename, &data).await {
            Ok(saved_filename) => {
                println!("Successfully uploaded: {} -> {} ({} bytes)", 
                    filename, saved_filename, data.len());
                
                uploaded_files.push(json!({
                    "field_name": field_name,
                    "original_name": filename,
                    "saved_name": saved_filename,
                    "size": data.len()
                }));
            }
            Err(e) => {
                eprintln!("Failed to save file {}: {:?}", filename, e);
                return Err(e.into());
            }
        }
    }

    if uploaded_files.is_empty() {
        return Ok(Json(json!({
            "message": "No files uploaded",
            "files": [],
            "count": 0
        })));
    }

    Ok(Json(json!({
        "message": "Files uploaded successfully",
        "files": uploaded_files,
        "count": uploaded_files.len()
    })))
}