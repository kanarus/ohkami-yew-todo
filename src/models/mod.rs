use ohkami::serde::{Deserialize, Serialize};
use ohkami::fang::JWTToken;


pub type ID = String;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct SignupResponse {
    pub token: JWTToken,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct CreateCardResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCard {
    pub title: String,
    pub todos: [Todo; Card::N_TODOS],
}
