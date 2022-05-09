use actix_web::{HttpResponse, Responder};
use paperclip::actix::{api_v2_operation, get};

/// Проверка работоспособности сервиса
#[api_v2_operation]
#[get("/api/alive")]
pub async fn alive() -> impl Responder {
    HttpResponse::Ok()
}
