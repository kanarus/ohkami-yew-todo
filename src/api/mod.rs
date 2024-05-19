pub mod errors;
pub mod jwt;
pub mod utils;

use self::jwt::JWTPayload;
use self::errors::ServerError;
use crate::Bindings;
use crate::models::{Card, SignupResponse, Todo, UpdateCard};
use web_sys::{wasm_bindgen::JsCast, WorkerGlobalScope, js_sys};
use ohkami::typed::status;
use ohkami::serde::Deserialize;
use ohkami::utils::unix_timestamp;
use ohkami::Memory;


#[worker::send]
pub async fn signup(
    b: Bindings,
) -> Result<SignupResponse, ServerError> {
    let user_id = WorkerGlobalScope::unchecked_from_js(js_sys::global().into())
        .crypto().unwrap().random_uuid();

    b.DB.prepare("INSERT INTO users (id) VALUES (?)")
        .bind(&[(&user_id).into()])?
        .run().await?;

    Ok(SignupResponse {
        token: jwt::new_token_for(user_id)
    })
}

#[worker::send]
pub async fn create_card(
    b:    Bindings,
    auth: Memory<'_, JWTPayload>,
) -> Result<status::Created<Card>, ServerError> {
    let id = WorkerGlobalScope::unchecked_from_js(js_sys::global().into())
        .crypto().unwrap().random_uuid();

    b.DB.prepare("INSERT INTO cards (id, user_id) VALUES (?1, ?2)")
        .bind(&[id.into(), (&auth.user_id).into()])?
        .run().await?;

    Ok(status::Created(todo!()))
}

#[worker::send]
pub async fn list_cards(
    b:    Bindings,
    auth: Memory<'_, JWTPayload>,
) -> Result<Vec<Card>, ServerError> {
    let card_records = {

    };

    todo!()

    /*
    let todo_records = {
        #[derive(Deserialize)] struct Record {
            id:            String,
            content:       String,
            completed_at:  Option<u64>,
            tag_ids_csv:   Option<String>,
            tag_names_csv: Option<String>,
        }
        b.DB.prepare("SELECT
            todos.id,
            todos.content,
            todos.completed_at,
            group_concat(tags.id) AS tag_ids_csv,
            group_concat(tags.name) AS tag_names_csv
        FROM todos
        LEFT OUTER JOIN todo_tags ON todo_tags.todo_id = todos.id
        LEFT OUTER JOIN tags      ON tags.id = todo_tags.tag_id
        WHERE user_id = ?1
        GROUP BY todos.id")
            .bind(&[(&auth.user_id).into()])?
            .all().await?.results::<Record>()?
    };

    Ok(todo_records.into_iter().map(|r| Todo {
        id:        r.id,
        content:   r.content,
        completed: r.completed_at.is_some(),
        tags: if r.tag_ids_csv.is_none() {vec![]} else {Iterator::zip(
            r.tag_ids_csv.unwrap().split(',').map(|id| id.parse().unwrap()),
            r.tag_names_csv.unwrap().split(',').map(|name| name.to_string())
        ).map(|(id, name)| Tag { id, name }).collect()}
    }).collect())
    */
}

#[worker::send]
pub async fn update_card(id: &str,
    b:    Bindings,
    auth: Memory<'_, JWTPayload>,
    req:  UpdateCard,
) -> Result<(), ServerError> {
    b.assert_user_is_owner_of_card(&auth.user_id, id).await?;

    /*
    let UpdateTodo { content, tags } = req;
    let tags = match tags {
        None        => None,
        Some(names) => Some(b.get_or_create_tags_by_names(&names).await?)
    };

    if content.is_none() && tags.is_none() {
        return Ok(())
    }

    b.DB.batch([
        content.map(|new_content| 
            b.DB.prepare("UPDATE todos SET content = ?1 WHERE id = ?2")
                .bind(&[new_content.into(), id.into()])
        ).transpose()?,
        tags.is_some().then(|| 
            b.DB.prepare("DELETE FROM todo_tags WHERE todo_id = ?")
                .bind(&[id.into()])
        ).transpose()?,
        tags.filter(|tags| tags.len() > 0).map(|new_tags|
            b.DB.prepare(format!("INSERT INTO todo_tags (
                todo_id, tag_id
            ) VALUES {}", vec!["(?,?)"; new_tags.len()].join(",")))
                .bind(&new_tags.iter()
                    .map(|tag| [id.into(), tag.id.into()])
                    .flatten().collect::<Vec<_>>()
                )
        ).transpose()?,
    ].into_iter().flatten().collect()).await?;
    */

    Ok(())
}
