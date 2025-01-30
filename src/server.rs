use actix_web::HttpResponse;
use actix_web::web::Json;
use serde::Serialize;
use serde::Deserialize;
use actix_web::web;

use crate::domain::Service;

pub const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Serialize)]
pub struct Response {
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register(req: Json<RegisterRequest>, service: web::Data<Service>) -> HttpResponse {
    if let Err(err) = service.register(&req.email, &req.password) {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .json(Response { message: err });
    }

    let response = Response {
        message: "Success ! ".to_string(),
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
}

#[post("/login")]
pub async fn login(req: Json<LoginRequest>, service: web::Data<Service>) -> HttpResponse {
    match service.login(&req.email, &req.password) {
        Ok(user) => {
            let response = Response {
                message: format!("Login successful. User id = {}, User = {}", user.id, user.email),
            };

            HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(response)
        }
        Err(err) => {
            HttpResponse::BadRequest()
                .content_type(APPLICATION_JSON)
                .json(Response { message: err })
        }
    }
}