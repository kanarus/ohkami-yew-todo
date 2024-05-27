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
#[derive(PartialEq, Clone)]
pub struct CreateCardRequest {
    pub title: String,
    pub todos: [String; Card::N_TODOS],
}
#[allow(unused)]
impl CreateCardRequest {
    pub fn empty() -> Self {
        Self {
            title: String::new(),
            todos: std::array::from_fn(|_| String::new()),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.title.is_empty() &&
        self.todos.iter().all(String::is_empty)
    }
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
