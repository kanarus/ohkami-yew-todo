use ohkami::serde::{Deserialize, Serialize};
use ohkami::typed::Payload;
use ohkami::builtin::{payload::JSON, item::JWTToken};


pub type ID = String;

#[Payload(JSON/SD)]
#[derive(PartialEq, Clone)]
pub struct Card {
    pub id:    ID,
    pub title: String,
    pub todos: [Todo; Self::N_TODOS],
}
impl Card {
    pub const N_TODOS: usize = 10;
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Todo {
    pub content:   String,
    pub completed: bool,
}

#[Payload(JSON/SD)]
pub struct SignupResponse {
    pub token: JWTToken,
}

#[Payload(JSON/SD)]
pub struct CreateCardRequest {
    pub init: CreateCardInit,
}
#[derive(Serialize, Deserialize)]
pub enum CreateCardInit {
    Title(String),
    Todo { index: usize, content: String },
}

#[Payload(JSON/SD)]
pub struct CreateCardResponse {
    pub id: String,
}

#[Payload(JSON/SD)]
pub struct UpdateCard {
    pub title: String,
    pub todos: [Todo; Card::N_TODOS],
}
