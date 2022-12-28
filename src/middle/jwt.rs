use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    error,
};
use futures_util::future::LocalBoxFuture;
use crate::utils::{constants, token};
use chrono::Local;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
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
        println!("Hi from start. You requested: {}", req.path());
        let mut _is_valid:bool = false;
        // Bypass some account routes
        let _ignore = is_ignore(req.path());
        if !_ignore {
            if let Some(_auth_value) = req.headers().get("Authorization") {
                if let Ok(raw_token) = _auth_value.to_str() {
                    let token_str = raw_token.trim_start_matches("Bearer ");
                    if let Ok(jwt_token)= token::decode_token(token_str.to_string()){
                        if jwt_token.claims.exp < Local::now().timestamp() {
                            _is_valid = true;
                        }
                    }
                }
            }
        }
        let fut = self.service.call(req);

        Box::pin(async move {
            let next = fut.await?;
            if _ignore || _is_valid {
               return Ok(next);
            } else {
               return Err(error::ErrorUnauthorized("401"));
            }
        })
    }
}

fn is_ignore(path: &str) -> bool {
      for ignore_route in constants::IGNORE_ROUTES.into_iter() {
            if path.starts_with(ignore_route) {
                return true
            }
        }
    false
}