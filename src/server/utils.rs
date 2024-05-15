use worker::D1Type;
use crate::{models::Tag, Bindings};


impl Bindings {   
    pub async fn get_or_create_tags_by_names(&self,
        tag_names: &[&str],
    ) -> Result<Vec<Tag>, worker::Error> {
        let __get_id__  = self.DB.prepare("SELECT id FROM tags WHERE name = ?1");
        let __add_tag__ = self.DB.prepare("INSERT INTO tags (name) VALUES (?1)");

        let mut tags = Vec::with_capacity(tag_names.len());

        // This is not so bad mannner because `tag_names` is already validated
        // to have at most 5 tag names
        for name in tag_names {
            tags.push(match __get_id__.bind_refs(&[D1Type::Text(name)])?.first::<i32>(Some("id")).await? {
                Some(id) => Tag { id: id as _, name: name.to_string() },
                None => {
                    __add_tag__.bind_refs(&[D1Type::Text(name)])?.run().await?;
                    let id = __get_id__.bind_refs(&[D1Type::Text(name)])?.first::<i32>(Some("id")).await?;
                    Tag { id: id.unwrap() as _, name: name.to_string() }
                }
            })
        }

        Ok(tags)
    }
}
