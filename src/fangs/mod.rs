use ohkami::serde::{Serialize, Deserialize};
use ohkami::builtin::fang::JWT;
use crate::Bindings;


#[derive(Serialize, Deserialize)]
pub struct JWTPayload {
    pub user_id: String,
    iat: u64,
}

pub fn jwt() -> JWT<JWTPayload> {
    JWT::default(Bindings::JWT_SECRET_KEY())
}
