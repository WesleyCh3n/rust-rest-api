use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    id: i32,
    num: i32,
}

#[get("/{id}/{num}")]
pub async fn get_id(i: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Get id: {} num: {}", i.id, i.num))
}
