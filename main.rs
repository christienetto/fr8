use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use sanitize_filename::sanitize;
use std::{fs::File, io::Write, path::PathBuf};

static UPLOAD_DIR: &str = "./uploads";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ensure upload directory exists
    std::fs::create_dir_all(UPLOAD_DIR)?;

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin() 
            .allow_any_method() 
            .allow_any_header() 
            .supports_credentials();

        App::new()
            .wrap(cors) 
            .wrap(middleware::Logger::default())
            .route("/upload", web::post().to(upload_file))
            .route("/ping", web::get().to(ping))
    })
    .bind(("127.0.0.1", 9424))?
    .run()
    .await
}

async fn upload_file(mut payload: Multipart) -> impl Responder {
    while let Some(item) = payload.next().await {
        match item {
            Ok(mut field) => {
                let content_disposition = field.content_disposition();
                let filename = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename())
                    .map(|name| sanitize(name))
                    .unwrap_or_else(|| "unknown".to_string());

                let filepath = PathBuf::from(UPLOAD_DIR).join(filename);

                let mut file = match File::create(&filepath) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Failed to create file: {}", e);
                        return HttpResponse::InternalServerError()
                            .body("Failed to create file");
                    }
                };

                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => {
                            if let Err(e) = file.write_all(&data) {
                                eprintln!("Failed to write file: {}", e);
                                return HttpResponse::InternalServerError()
                                    .body("Failed to write file");
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read chunk: {}", e);
                            return HttpResponse::InternalServerError()
                                .body("Failed to read file data");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error processing field: {}", e);
                return HttpResponse::BadRequest().body("Invalid multipart data");
            }
        }
    }

    HttpResponse::Ok().body("File uploaded successfully")
}

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
