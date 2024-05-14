use ohkami::typed::Payload;
use ohkami::builtin::{payload::JSON, item::JWTToken};


#[Payload(JSON/S)]
pub struct SigninResponse {
    pub token: JWTToken,
}

#[Payload(JSON/S where Self::validate)]
pub struct Todo {
    pub id:        String,
    pub content:   String,
    pub completed: bool,
    pub tags:      Vec<Tag>,
}
impl Todo {
    const TAGS_LIMIT: usize = 5;

    fn validate(&self) -> Result<(), String> {
        (self.tags.len() <= Self::TAGS_LIMIT).then_some(())
            .ok_or_else(|| format!("One todo can have at most five tags"))?;

        Ok(())
    }
}

#[Payload(JSON/S)]
pub struct Tag {
    pub id:    usize,
    pub label: String,
}
