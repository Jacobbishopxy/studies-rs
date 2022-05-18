//! Actix-web example for uploading and downloading files
//!
//! [Original](https://medium.com/swlh/build-your-first-rest-api-using-rust-language-and-actix-framework-8f827175b30f)

use std::{
    fs::{create_dir_all, read, File},
    io::Write,
    path::Path,
    time::SystemTime,
};

use actix_multipart::Multipart;
use actix_web::{
    http::{header::ContentType, StatusCode},
    middleware::Logger,
    web, App, HttpResponse, HttpServer, ResponseError, Result,
};
use derive_more::{Display, Error};
use futures::{StreamExt, TryStreamExt};
use log::{info, LevelFilter};
use serde::{Deserialize, Serialize};

const FILE_DIR: &str = "./tmp";

#[derive(Serialize, Deserialize)]
struct UploadResponse {
    filename: String,
    time: u64,
    size: u64,
}

#[derive(Deserialize)]
struct DownloadRequest {
    filename: String,
}

#[derive(Serialize, Debug, Display, Error)]
enum AppError {
    #[display(fmt = "File not found: {}", f)]
    FileNotFound { f: String },

    #[display(fmt = "File cannot be created: {}", f)]
    FileCannotBeCreated { f: String },

    #[display(fmt = "Unknown error: {}", e)]
    Unknown { e: String },
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::FileNotFound { .. } => StatusCode::BAD_REQUEST,
            AppError::FileCannotBeCreated { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unknown { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

async fn upload(mut payload: Multipart) -> Result<HttpResponse> {
    // 预创建文件路径
    create_dir_all(FILE_DIR)?;

    let mut upload_responses = Vec::<UploadResponse>::new();

    // 遍历所有 Multipart 数据
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();

        // 只处理 Multipart 数据中 "file" 作为 key 的数据
        if let Some("file") = content_type.get_name() {
            let filename = content_type.get_filename().unwrap_or("unknown");
            info!("uploading: {}", filename);

            let f_n = filename.to_string();

            let filepath = format!("{}/{}", FILE_DIR, sanitize_filename::sanitize(filename));
            // File::create 为阻塞函数，使用 web::block 来包装它
            let mut f = web::block(|| File::create(filepath))
                .await
                .map_err(|e| AppError::FileCannotBeCreated { f: e.to_string() })??;

            // 将 Multipart 数据写入文件
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // 文件写入为阻塞操作，使用 web::block 来包装它
                f = web::block(move || f.write_all(&data).map(|_| f)).await??;
            }

            let time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            upload_responses.push(UploadResponse {
                filename: f_n,
                time,
                size: f.metadata().unwrap().len(),
            });
        }
    }

    Ok(HttpResponse::Ok().json(&upload_responses))
}

async fn download(info: web::Path<DownloadRequest>) -> Result<HttpResponse> {
    info!("downloading: {}", info.filename);

    let path = format!("{}/{}", FILE_DIR, info.filename);
    let p = path.clone();
    if !Path::new(&path).exists() {
        return Ok(AppError::FileNotFound {
            f: info.filename.clone(),
        }
        .error_response());
    }

    let data = web::block(|| read(p).unwrap())
        .await
        .map_err(|e| AppError::Unknown { e: e.to_string() })?;

    Ok(HttpResponse::Ok()
        .append_header(ContentType::octet_stream())
        .body(data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "actix_server=info,actix_web=info,actix_upload_download=info",
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    log::set_max_level(LevelFilter::Debug);

    let bind = ("127.0.0.1", 8080);
    info!("Starting server...");
    info!("Starting server on {:?}", &bind);

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new().wrap(logger).service(
            web::scope("/api")
                .route("/files", web::post().to(upload))
                .route("/files/{filename}", web::get().to(download)),
        )
    })
    .bind(bind)?
    .run()
    .await
}
