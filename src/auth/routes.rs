use actix_web::web;

use crate::auth::handler::{
    register,
    login
};

pub fn scoped_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(register))
            .route("/signin", web::post().to(login))
    );
}