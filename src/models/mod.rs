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
            name.chars().all(|char| char.is_ascii_lowercase())
        ).then_some(())
        .ok_or_else(|| format!("Each tag name must be non-empty and consist of 'a'~'z'"))?;

    Ok(())
}

#[Payload(JSON/S)]
pub struct SigninResponse {
    pub token: JWTToken,
}

#[Payload(JSON/S where validate_tag_names(self.tags.iter().map(|tag| &*tag.name).collect::<Vec<_>>()))]
pub struct Todo {
    pub id:        String,
    pub content:   String,
    pub completed: bool,
    pub tags:      Vec<Tag>,
}

#[Payload(JSON/S)]
pub struct Tag {
    pub id:   usize,
    pub name: String,
}

#[Payload(JSON/D where validate_tag_names(&self.tags))]
pub struct CreateTodo<'req> {
    pub content: &'req str,
    pub tags:    Vec<&'req str>,
}
