use serde::Deserialize;


#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserRegisterRequest {
    pub username: String,
    pub password: String,
    pub repeat_password: String,
    pub email: Option<String>,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct BillCreateRequest {
    pub title: String,
    pub description: Option<String>,
    pub total_amount: i32,
    pub category: String,
    pub days_till_due: i64,
}

#[derive(Deserialize)]
pub struct BilleeCreateRequest {
   pub name: String,
   pub percentage: i32,
   pub user_id: Option<i32>
}
