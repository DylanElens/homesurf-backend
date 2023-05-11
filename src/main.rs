use actix_cors::Cors;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_multipart_demo::models::File;
use actix_multipart_demo::{create_file, establish_connection, list_files, delete_file, get_files_by_id};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use futures_util::TryStreamExt as _;
use serde::Deserialize;
use std::io::Result;
use std::path::{Path, PathBuf};
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
            .route("/files", web::get().to(file_list))
            .route("/bulk-download-files", web::post().to(bulk_download_files))
            .route("/files/{id}", web::delete().to(delete_file_controller))
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

            match create_file(
                &mut conn,
                field.content_disposition().get_filename().unwrap(),
                &destination,
                &field.content_type().unwrap().to_string(),
            ) {
                Ok(_) => print!(""),
                Err(_) => println!(),
            }

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
    let path: PathBuf = PathBuf::from(format!(
        "{}{}",
        "./upload/".to_owned().as_str(),
        req.match_info().query("filename")
    ));
    println!("here we are {:?}", path);
    Ok(NamedFile::open(path)?)
}

async fn delete_file_controller(_: HttpRequest, file_id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = establish_connection();
    match delete_file(&mut conn, &file_id) {
        Ok(_) => Ok(HttpResponse::Ok().into()),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    }
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


#[derive(Deserialize, Debug)]
struct Info {
    file_ids: Vec<i32>,
}

async fn bulk_download_files(payload: web::Json<Info>) -> Result<HttpResponse> {
    let mut conn = establish_connection();
    let files: Vec<File> = get_files_by_id(&mut conn, &payload.file_ids).unwrap();
    println!("files {:?}", files);


    Ok(HttpResponse::Ok().into())
}

async fn file_list() -> Result<HttpResponse> {
    let mut conn = establish_connection();
    match list_files(&mut conn) {
        Ok(files) => {
            Ok(HttpResponse::Ok().json(files))
        },
        _ => Ok(HttpResponse::UnprocessableEntity().into())
    }
}
