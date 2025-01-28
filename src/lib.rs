use actix_files::NamedFile;
use actix_files as af;
use actix_web::{web, Result, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::dev::Server;
use std::path::PathBuf;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn index_file(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./files/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(af::Files::new("/static", ".")
                .show_files_listing()
                .use_last_modified(true),
            )
            .route("/health_check", web::get().to(health_check))
            .route("/index", web::get().to(index_file))
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    Ok(server)
}
