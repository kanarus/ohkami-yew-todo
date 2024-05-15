use super::Bindings;
use ohkami::serde::{Serialize, Deserialize};
use ohkami::builtin::{fang::JWT, item::JWTToken};
use ohkami::utils::unix_timestamp;


#[derive(Serialize, Deserialize)]
pub struct JWTPayload {
    pub user_id: String,
    iat: u64,
}

pub fn fang() -> JWT<JWTPayload> {
    JWT::default(Bindings::JWT_SECRET_KEY())
}

pub fn new_token_for(user_id: String) -> JWTToken {
    self::fang().issue(JWTPayload { user_id, iat: unix_timestamp() })
}
