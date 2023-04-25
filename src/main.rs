use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{HttpServer, HttpRequest, HttpResponse, App, web};
use futures_util::TryStreamExt as _;
use std::io::Result;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use uuid::Uuid;

#[actix_web::main]
async fn main() -> Result<()> {
    if !Path::new("./upload").exists() {
        fs::create_dir("./upload").await?;
    }

    HttpServer::new(|| {
        let cors = Cors::permissive().allow_any_origin();
        App::new()
            .wrap(cors)
            .route("/", web::get().to(healthcheck))
            .route("/upload", web::post().to(upload))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn healthcheck() -> String {
    "Server is up and running.".to_string()
}

async fn upload(mut payload: Multipart, _req: HttpRequest) -> HttpResponse {
    let dir: &str = "./upload/";
    loop {
        if let Ok(Some(mut field)) = payload.try_next().await {

            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );

            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }
        } else {
            break;
        }
    }
    HttpResponse::NoContent().into()
}
