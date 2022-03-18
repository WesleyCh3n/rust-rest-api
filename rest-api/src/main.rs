mod api;
mod models;

use crate::api::hardware_controller::{
    create_id, delete_id, get, get_id, update_id,
};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body(
            r#"
                <title>Home</title>
                <body>
                    <h2>This is Home Page</h2>
                    <p>use /api/ to test out CRUD restful api</p>
                </body>
            "#,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(home).service(
            web::scope("/api")
                .service(create_id)
                .service(get)
                .service(get_id)
                .service(update_id)
                .service(delete_id),
        )
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
