use validator::Validate;
use sqlx::{
    error::Error as SqlxError,
    Row
};
use argon2::{
    password_hash::{
        SaltString,
        rand_core::OsRng,
        PasswordHash
    },
    Argon2,
    PasswordHasher, 
    PasswordVerifier
};

use crate::{
    db::DbPool,
    errors::{
        AppError,
        JsonErrorMessage
    },
    auth::model::{
        AuthRegister,
        AuthLogin
    }
};

pub async fn register_service(
    db_pool: &DbPool, 
    payload: AuthRegister
) -> Result<String, AppError> 
{
    /* * validating user input */
    payload.validate()?;
    /* * end validating user input */

    /* * simulate error */
    let fake_error = false; 
    if fake_error { return Err(AppError::from(SqlxError::RowNotFound)); }
    /* * end simulate error */

    /* * sql query check username is exist? */
    let sql_query = sqlx::query("SELECT username FROM credentials WHERE username = ?");
    let query_result = sql_query
        .bind(&payload.username)
        .fetch_one(db_pool)
        .await
        .is_ok();
    if query_result {
        let json_error_message: JsonErrorMessage<Option<String>> = JsonErrorMessage {
            code: 409,
            message: format!("username '{}' is already taken. please choose another!", &payload.username),
            details: None
        };
        return Err(
            AppError::Conflict(json_error_message.into())
        );
    }
    /* * end sql query check username is exist? */

    /* * hashing user payload password*/
    let password = payload.password.as_bytes();
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();

    let hashed_password = argon2.hash_password(password, &salt)
        .map_err(|err| {
            let json_error_message = JsonErrorMessage {
                code: 500,
                message: String::from("error hashing password"),
                details: Some::<String>(err.to_string())
            };

            AppError::InternalServerError(json_error_message.into())
        })?
        .to_string();
    /* * end hashing user payload passsword */

    /* * sql query insert user to database */
    let sql_query = sqlx::query("INSERT INTO credentials (username, password) VALUES (?, ?);");
    let _query_result = sql_query
        .bind(&payload.username)
        .bind(hashed_password)
        .execute(db_pool)
        .await?;
    /* * end sql query insert user to database */

    /* * sql query check username is already stored */
    let sql_query = sqlx::query("SELECT username FROM credentials WHERE username = ?");
    let query_result = sql_query
        .bind(&payload.username)
        .fetch_one(db_pool)
        .await?;
    let stored_user: &str = query_result.get("username");
    /* * end sql query check username is already stored */

    if payload.username.eq(&stored_user) {
        Ok(
            payload.username.clone()
        )
    } else {
        /* * * rarely trigged */
        let json_error_message: JsonErrorMessage<Option<String>> = JsonErrorMessage {
            code: 500,
            message: format!("the registration for user {} has encountered a failure.", stored_user),
            details: None
        };
        Err(
            AppError::InternalServerError(json_error_message.into())
        )
        /* * * end rarely trigged */
    } 
}


pub async fn login_service(
    db_pool: &DbPool, 
    payload: AuthLogin
) -> Result<String, AppError>
{
    /* * validating user input */
    payload.validate()?;
    /* * end validating user input */

    /* * sql query check user is exist? in database */
    let sql_query = sqlx::query("SELECT password FROM credentials WHERE username = ?");
    let query_result = sql_query
        .bind(&payload.username)
        .fetch_one(db_pool)
        .await
        .map_err(|err| {
            let json_error_message = JsonErrorMessage {
                code: 401,
                message: format!("login failed for user '{}'. please double-check your credentials info.", &payload.username),
                details: Some(err.to_string())
            };
            AppError::Unauthorized(json_error_message.into())
        })?;
    let stored_hash_password: &str = query_result.get("password");
    /* * end sql query check user is exist? in database */

    /* * verify stored user password */
    let parsed_stored_hash_password = PasswordHash::new(&stored_hash_password)
        .map_err(|err| {
            let json_error_message = JsonErrorMessage {
                code: 500,
                message: String::from("failed to parse hashed_password"),
                details: Some(err.to_string())
            };
            AppError::InternalServerError(json_error_message.into())
        })?;
    let argon2 = Argon2::default();
    
    let user_payload_password = payload.password.as_bytes();
    let _verified_password = argon2
        .verify_password(
            user_payload_password,
            &parsed_stored_hash_password)
        .map_err(|err| {
            let json_error_message = JsonErrorMessage {
                code: 401,
                message: format!("login failed for user '{}'. please double-check your credentials info.", &payload.username),
                details: Some(err.to_string())
            };
            AppError::Unauthorized(json_error_message.into())
        })?;
    /* * end verify stored user password */

    Ok(payload.username)
}