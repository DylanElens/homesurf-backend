use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_multipart_demo::{create_file, establish_connection, list_files};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use futures_util::TryStreamExt as _;
use std::io::Result;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use uuid::Uuid;
use actix_files::NamedFile;

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
            .route("/files", web::get().to(upload))
            .route("/files/{filename:.*}", web::get().to(get_file))
            .route("/download/{filename:.*}", web::get().to(download_file))
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
    let mut conn = establish_connection();
    loop {
        if let Ok(Some(mut field)) = payload.try_next().await {
            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );
            let result = create_file(
                &mut conn,
                field.content_disposition().get_filename().unwrap(),
                &destination,
                &field.content_type().unwrap().to_string(),
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

async fn get_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from(format!("{}{}","./upload/".to_owned().as_str() ,req.match_info().query("filename")));
    println!("here we are {:?}", path);
    Ok(NamedFile::open(path)?)
}




async fn download_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from(format!(
        "{}{}",
        "./upload/".to_owned().as_str(),
        req.match_info().query("filename")
    ));
    println!("here we are {:?}", path);
    Ok(NamedFile::open(path)?.set_content_disposition(
        actix_web::http::header::ContentDisposition {
            disposition: "attachment".into(),
            parameters: Vec::new(),
        },
    ))
}

