#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub session: String,
}