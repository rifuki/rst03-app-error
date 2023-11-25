use serde::Deserialize;
use validator::Validate;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[0-9a-zA-Z]{2,}$").unwrap();
}

#[derive(Deserialize)]
pub struct In<T> {
    pub user: T
}

#[derive(Deserialize, Validate)]
pub struct AuthRegister {
    #[validate(
        length(
            min = 2,
            max = 25,
            message = "username is invalid - must be 2-25 characters only"
        ),
        regex(
            path = "RE_USERNAME",
            message = "username is invalid - must be only alphanumeric characters"
        )
    )]
    pub username: String,
    #[validate(
        length(
            min = 8,
            max = 75,
            message = "password is invalid - must have minimum length of 8 characters"
        )
    )]
    pub password: String,
    #[validate(
        must_match(
            other = "password",
            message = "confirm password is not same as password"
        )
    )]
    pub confirm_password: String
}

#[derive(Deserialize, Validate)]
pub struct AuthLogin {
    #[validate(
        length(
            min = 2,
            max = 25,
            message = "username is invalid - must be only 2-25 characters"
        ),
        regex(
            path = "RE_USERNAME",
            message = "username is invalid - must be only alphanumeric/underscore characters"
        )
    )]
    pub username: String,
    #[validate(
        length(
            min = 8,
            max = 75,
            message = "password is invalid - must have minimum length of 8 characters"
        )
    )]
    pub password: String
}