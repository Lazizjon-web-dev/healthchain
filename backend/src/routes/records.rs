use actix_web::{
    web::{get, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/records"));
}
