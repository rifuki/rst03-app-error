use actix_web::{
    HttpServer, 
    App, 
    web, 
    middleware
};
use color_print::cprintln;

use rst03_app_error::{
    establish_connection,
    scoped_auth
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap_or_else(|err| {
        eprintln!("dotenv err [{}]", err);
        std::process::exit(1);
    });

    let app_port = std::env::var("APP_PORT")
        .unwrap_or(String::from("80"));

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_err| {
        let error_message = "\"DATABASE_URL\" must be first";
        cprintln!("<bold,red>{}</bold,red>", error_message.to_uppercase());
        std::process::exit(1);
    });
    let db_pool = establish_connection(&db_url).await;
    let app_state = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(
                middleware::NormalizePath::trim()
            )
            .service(
                web::scope("/api")
                    .configure(scoped_auth)
            )
    })
    .bind(format!("0.0.0.0:{}", app_port))?
    .run()
    .await
}