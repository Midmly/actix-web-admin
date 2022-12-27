mod utils;
mod model;
mod middle;
mod service;
mod controller;
mod config;

#[macro_use]
extern crate serde_derive;
extern crate erased_serde;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate futures;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web, middleware};

// 通过宏标注指出请求路径和方法
#[get("/private/t1")]
async fn get_request() -> impl Responder {
    HttpResponse::Ok().body("get_ok")
}

// 把请求体提取成String
async fn post_request(body: String) -> impl Responder {
    println!("{}", body);
    HttpResponse::Ok().body("post_ok")
}

/// 测试链接：
/// post@/t1
/// get@/t1
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/private")
                .wrap(middle::jwt::Authentication)
                    .route("", web::get().to(post_request)),
            )
            // 在这里传入定义的服务
            .service(get_request)
            // 这里注意到，route接收三个参数：路径，请求方法和handler
            .route("/t1", web::post().to(post_request))
    })
    .bind("127.0.0.1:8190")?
    .run()
    .await
}