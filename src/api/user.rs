use actix_web::{HttpResponse, Error, web};
use crate::model::user::LoginRequest;
use crate::utils::{response::ResponseBody, token::generate_token};

pub async fn login(l_req: web::Json<LoginRequest>) -> Result<HttpResponse, Error> {
if !l_req.password.eq("admin"){
   return Ok(HttpResponse::BadRequest().json(ResponseBody::new("bad password", String::new())));
}
if !l_req.username.eq("admin"){
    return Ok(HttpResponse::BadRequest().json(ResponseBody::new("bad username", String::new())));
 }
 let token_str = generate_token(l_req.username.clone());
   Ok(HttpResponse::Ok().json(ResponseBody::new(String::new().as_str(), token_str)))
}