use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello, {}! id:{}", name, id)
}

#[get("/")]
async fn root() -> impl Responder {
    format!("ooh you found the root page id")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting up ...");
    HttpServer::new(|| App::new().service(root).service(index))
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
