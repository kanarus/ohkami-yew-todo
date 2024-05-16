use ohkami::typed::Payload;
use ohkami::builtin::{payload::JSON, item::JWTToken};


fn validate_tag_names<'name>(tag_names: impl AsRef<[&'name str]>) -> Result<(), String> {
    const TAGS_LIMIT: usize = 5;

    let tag_names = tag_names.as_ref();

    let _: () = (tag_names.len() <= TAGS_LIMIT).then_some(())
        .ok_or_else(|| format!("One todo can't have more than {TAGS_LIMIT} tags"))?;

    let _: () = tag_names.iter()
        .all(|name|
            (1..=32).contains(&name.len()) &&
            name.chars().all(|char| matches!(char, 'a'..='z' | '0'..='9' | '-' | '_'))
        ).then_some(())
        .ok_or_else(|| format!("Each tag name must consist of [a-z, 0-9, -, _] and the length must be 1~32."))?;

    Ok(())
}


#[Payload(JSON/SD)]
pub struct SignupResponse {
    pub token: JWTToken,
}

#[Payload(JSON/SD where validate_tag_names(self.tags.iter().map(|tag| &*tag.name).collect::<Vec<_>>()))]
pub struct Todo {
    pub id:        String,
    pub content:   String,
    pub completed: bool,
    pub tags:      Vec<Tag>,
}

#[Payload(JSON/SD)]
pub struct Tag {
    pub id:   usize,
    pub name: String,
}

#[Payload(JSON/SD where validate_tag_names(&self.tags))]
pub struct CreateTodo<'req> {
    pub content: &'req str,
    pub tags:    Vec<&'req str>,
}

#[Payload(JSON/SD where validate_tag_names(self.tags.as_deref().unwrap_or_default()))]
pub struct UpdateTodo<'req> {
    pub content: Option<&'req str>,
    pub tags:    Option<Vec<&'req str>>,
}
