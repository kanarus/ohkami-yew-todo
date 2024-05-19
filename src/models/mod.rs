use ohkami::serde::{Deserialize, Serialize};
use ohkami::typed::Payload;
use ohkami::builtin::{payload::JSON, item::JWTToken};


#[Payload(JSON/SD)]
#[derive(PartialEq, Clone)]
pub struct Card {
    pub id:    String,
    pub title: String,
    pub todos: [Option<Todo>; Self::TODO_LIMIT],
}
impl Card {
    pub const TODO_LIMIT: usize = 10;
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Todo {
    pub id:        String,
    pub content:   String,
    pub completed: bool,
}

#[Payload(JSON/SD)]
pub struct SignupResponse {
    pub token: JWTToken,
}

#[Payload(JSON/SD)]
pub struct UpdateCard {
    pub title: String,
    pub todos: [Option<Todo>; Card::TODO_LIMIT],
}
