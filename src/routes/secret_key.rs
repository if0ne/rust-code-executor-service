/// Обработка запроса
/// Вытягивается из хедера (x-api-key) запроса секретный ключ
/// При неудачи выкидывается BadRequest
/// Как реализовано смотреть: https://actix.rs/docs/middleware/
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::LocalBoxFuture;
use lazy_static::lazy_static;
use std::future::{ready, Ready};

lazy_static! {
    pub static ref SECRET_KEY: String =
        base64::encode(std::env::var("SECRET_KEY").unwrap().as_bytes());
}

pub struct SecretKey;
pub struct SecretKeyMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for SecretKey
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SecretKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecretKeyMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for SecretKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(secret_key) = req
            .headers()
            .get("x-api-key")
            .and_then(|secret_key| secret_key.to_str().ok())
        {
            if secret_key == *SECRET_KEY {
                let future = self.service.call(req);
                Box::pin(async move {
                    let res = future.await?;
                    Ok(res)
                })
            } else {
                Box::pin(async move { Err(actix_web::error::ErrorBadRequest("Wrong secret key")) })
            }
        } else {
            Box::pin(async move { Err(actix_web::error::ErrorBadRequest("Need a secret key")) })
        }
    }
}
