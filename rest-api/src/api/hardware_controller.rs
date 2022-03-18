use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::models::hardware::Hardware;

#[post("/hardware")]
pub async fn create_id(hw: web::Json<Hardware>) -> impl Responder {
    HttpResponse::Ok().body(format!("Get id: {}", hw.id))
    // format!("Get id: {} num: {}", i.id, i.num)
}

#[get("/hardware")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok().body(format!("Get all id info"))
    // format!("Get id: {} num: {}", i.id, i.num)
}

#[get("/hardware/{id}")]
pub async fn get_id(i: web::Path<Hardware>) -> impl Responder {
    HttpResponse::Ok().body(format!("Get id: {}", i.id))
    // format!("Get id: {} num: {}", i.id, i.num)
}

#[put("/hardware/{id}")]
pub async fn update_id(
    i: web::Path<i32>,
    hw: web::Json<Hardware>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Get id: {}, update with: {:?}", i, hw))
    // format!("Get id: {} num: {}", i.id, i.num)
}

#[delete("/hardware/{id}")]
pub async fn delete_id(i: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("{} is deleted", i))
    // format!("Get id: {} num: {}", i.id, i.num)
}
