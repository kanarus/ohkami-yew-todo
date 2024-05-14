use ohkami::typed::Payload;
use ohkami::builtin::{payload::JSON, item::JWTToken};


#[Payload(JSON/S)]
pub struct SigninResponse {
    pub token: JWTToken,
}

#[Payload(JSON/S)]
pub struct Todo {
    pub id:        String,
    pub content:   String,
    pub completed: bool,
    pub tags:      Vec<Tag>,
}

#[Payload(JSON/S)]
pub struct Tag {
    pub id:    usize,
    pub label: String,
}
