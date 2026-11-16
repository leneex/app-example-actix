use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello world"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Порт передаёт платформа в переменной PORT; без неё — 7001
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(7001);

    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
