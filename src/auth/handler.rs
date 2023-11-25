use actix_web::{
    Responder, 
    HttpResponse, 
    web
};
use serde_json::json;

use crate::{
    db::DbPool,
    auth::{
        model::{
            In,
            AuthRegister,
            AuthLogin
        },
        services::{
            register_service,
            login_service
        }
    }
};

pub async fn register(
    app_state: web::Data<DbPool>,
    payload: web::Json<In<AuthRegister>>
) -> impl Responder 
{
    let db_pool = app_state.get_ref();
    let payload = payload.into_inner().user;

    let register_service = register_service(db_pool, payload).await;
    match register_service {
        /* * return success http response */
        Ok(user) => {
            let success_response = json!({
                "code": 201,
                "message": format!("user {} successfully registered", user)
            });

            HttpResponse::Created()
                .json(success_response)
        }
        /* * end return success http response */

        /* * return AppError http response */
        Err(app_error) => HttpResponse::from_error(app_error)
        /* * end return AppError http response */
    }
}

pub async fn login(
    app_state: web::Data<DbPool>,
    payload: web::Json<In<AuthLogin>>
) -> impl Responder 
{
    let db_pool = app_state.get_ref();
    let payload = payload.into_inner().user;

    let login_service = login_service(db_pool, payload).await;

    /* * return AppError http response */
    if let Err(app_error) = login_service {
       return HttpResponse::from_error(app_error);
    } 
    /* * end return AppError http response */

    /* * return success http response */
    let success_response = json!({
        "code": 200,
        "message": format!("user {} successfully logged", login_service.unwrap())
    });
    HttpResponse::Ok()
        .json(success_response)
    /* * end return success http response */

}