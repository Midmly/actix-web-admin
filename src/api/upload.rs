use actix_web::post;
use actix_web::{web::Path, HttpResponse, Error};
use crate::utils::files::{file_add_buf,file_create};
use crate::utils::response::ResponseBody;
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;


#[post("/upload/{name}")]
async fn upload_file(name: Path<String>, mut payload: Multipart) -> Result<HttpResponse, Error> {
    match file_create(name.to_string()) {
        Ok(_) => {
          // iterate over multipart stream
         while let Some(item) = payload.next().await {
            let mut field = item?;
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                if let Err(_e) = file_add_buf(name.to_string(),data) {
                    log::warn!(target: "upload_file","{}", _e.to_string());
                    return Ok(HttpResponse::BadRequest().json(ResponseBody::new(_e.to_string().as_str(), String::new())))
                    }
                }
            }
          Ok(HttpResponse::Ok().json(ResponseBody::new(String::new().as_str(), "OK")))
        },
        Err(_e) => Ok(HttpResponse::BadRequest().json(ResponseBody::new(_e.to_string().as_str(), String::new())))
    }
    
}