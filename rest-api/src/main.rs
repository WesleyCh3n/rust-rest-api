mod api;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use crate::api::driver_controller::get_id;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("This is home")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(web::scope("/api").service(get_id))
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
