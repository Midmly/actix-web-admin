use actix_web::{
    HttpResponse,
    http::StatusCode,
};

#[derive(Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}

pub struct GenerateResponse<T> {
    pub http_status: StatusCode,
    pub body: ResponseBody<T>,
}

impl<T: serde::Serialize> GenerateResponse<T> {
    pub fn new_200(data: T) -> GenerateResponse<T> {
        GenerateResponse {
            http_status: StatusCode::OK,
            body: ResponseBody {
                message: String::new(),
                data,
            }
        }
    }
    pub fn new_other(http_status: StatusCode, message: String) -> GenerateResponse<String> {
        GenerateResponse {
            http_status: StatusCode::OK,
            body: ResponseBody {
                message: String::new(),
                data: String::new(),
            }
        }
    }
    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}