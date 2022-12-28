use crate::model::user::{UserToken, LoginRequest};
use uuid::Uuid;
use chrono::Local;
// use actix_web::{web};
use jsonwebtoken::{TokenData, Validation, Header, DecodingKey, EncodingKey};
use crate::config::cfg;

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(cfg::KEY.as_ref()), &Validation::default())
}

pub fn generate_token(username: String) -> String {
    let now = Local::now().timestamp();
    let payload = UserToken {
        iat: now,
        exp: now + cfg::ONE_WEEK,
        user: username,
        session: Uuid::new_v4().to_string(),
    };

    jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(cfg::KEY.as_ref())).unwrap()
}
