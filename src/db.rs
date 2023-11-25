use sqlx::mysql::{
    MySqlPool,
    MySqlPoolOptions
};
use std::time::Duration;
use color_print::cprintln;

pub type DbPool = MySqlPool;

pub async fn establish_connection(db_url: &str) -> DbPool {
    MySqlPoolOptions::new()
        .acquire_timeout(Duration::from_millis(1000))
        .idle_timeout(Duration::from_millis(1))
        .max_connections(1)
        .connect(db_url)
        .await
        .unwrap_or_else(|_err| {
            let error_message = "failed to create pool connection";
            cprintln!("<red,bold>{}</red,bold>", error_message.to_uppercase());
            std::process::exit(1);
        })
} 
