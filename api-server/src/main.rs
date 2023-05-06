use actix_files::NamedFile;
use actix_web::{
    get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use std::path::PathBuf;
mod model;
mod routes;

#[get("/static/{filename:.*}")]
async fn serve_static_file(req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut file_path = String::from("./src/static/");
    file_path.push_str(path.to_str().unwrap_or(""));
    println!("file_path is {file_path}");
    Ok(NamedFile::open(file_path)?)
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("echo start {}", req_body);
    HttpResponse::Ok().body(req_body)
}

async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./src/static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .configure(routes::router::binding)
            .service(serve_static_file)
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(echo)
            .route("/{_:.*}", web::get().to(index)) // all other routes
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
