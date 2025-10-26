use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct UserRegisterRequest {
    pub username: String,
    pub password: String,
    pub repeat_password: String
}
