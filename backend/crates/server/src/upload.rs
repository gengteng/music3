//! # 上传

use axum::response::IntoResponse;

use crate::error::Result;
use axum::{extract::multipart::Multipart, response::Html};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
/// 上传文件
pub async fn upload(mut multipart: Multipart) -> Result<impl IntoResponse> {
    while let Some(field) = multipart.next_field().await? {
        let unique_filename = format!("uploads/{}.mp3", Uuid::new_v4());

        let data = field.bytes().await?;
        let mut file = File::create(unique_filename).await?;
        file.write_all(&data).await?;
    }

    Ok(Html("<h1>File uploaded successfully!</h1>"))
}
