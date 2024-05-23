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
use yew::html::IntoPropValue;


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

    b.DB.batch(vec![
        b.DB.prepare("INSERT INTO cards (id, user_id, created_at) VALUES (?1, ?2, ?3)")
            .bind(&[(&id).into(), (&auth.user_id).into(), (unix_timestamp() as usize).into()])?,
        b.DB.prepare(format!("INSERT INTO todos (card_id) VALUES {}", ["(?)"; Card::N_TODOS].join(",")))
            .bind(&std::array::from_fn::<_, {Card::N_TODOS}, _>(|_| (&id).into()))?,
    ]).await?;

    Ok(status::Created(Card {
        id,
        title: String::new(),
        todos: [(); Card::N_TODOS].map(|_| Todo {
            content:   String::new(),
            completed: false,
        }),
    }))
}

#[worker::send]
pub async fn list_cards(
    b:    Bindings,
    auth: Memory<'_, JWTPayload>,
) -> Result<Vec<Card>, ServerError> {
    let card_records = {
        #[derive(Deserialize)] struct Record {
            id:    String,
            title: String,
        }
        b.DB.prepare("SELECT id, title FROM cards WHERE user_id = ? ORDER BY created_at ASC")
            .bind(&[(&auth.user_id).into()])?
            .all().await?.results::<Record>()?
    };

    let mut todo_records = {
        #[derive(Deserialize)] struct Record {
            card_id:      String,
            content:      String,
            completed_at: Option<u64>,
        }
        let mut records = b.DB.prepare(format!(
            "SELECT card_id, content, completed_at FROM todos WHERE card_id IN ({})",
            ["?"].join(",")))
            .bind(&card_records.iter().map(|r| (&r.id).into()).collect::<Vec<_>>())?
            .all().await?.results::<Record>()?;
        records.sort_unstable_by(|a, b| str::cmp(&a.card_id, &b.card_id));
        records
    };

    Ok(card_records.into_iter().map(|r| {
        let mut todos = unsafe {use std::mem::MaybeUninit;
            // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
            MaybeUninit::<[MaybeUninit<Todo>; Card::N_TODOS]>::uninit().assume_init()
        };

        let todos_head = todo_records.iter()
            .step_by(Card::N_TODOS)
            .position(|t| t.card_id == r.id).unwrap();

        if todos_head + Card::N_TODOS == todo_records.len() {
            for i in (0..Card::N_TODOS).rev() {
                let r = todo_records.pop().unwrap();
                todos[i].write(Todo {
                    content:   r.content,
                    completed: r.completed_at.is_some()
                });
            }
        } else {
            for i in (todos_head..(todos_head + Card::N_TODOS)).rev() {
                let r = todo_records.swap_remove(i);
                todos[i].write(Todo {
                    content:   r.content,
                    completed: r.completed_at.is_some()
                });
            }
        }

        Card {
            id:    r.id,
            title: r.title,
            todos: todos.map(|mu| unsafe {mu.assume_init()})
    }}).collect())
}

#[worker::send]
pub async fn update_card(id: &str,
    b:    Bindings,
    auth: Memory<'_, JWTPayload>,
    req:  UpdateCard,
) -> Result<(), ServerError> {
    b.assert_user_is_owner_of_card(&auth.user_id, id).await?;

    let UpdateCard { title, todos } = req;

    let todo_ids_csv = b.DB.prepare("SELECT todo_ids_csv FROM cards WHERE id = ?")
        .bind(&[id.into()])?.first::<usize>(Some("todo_ids_csv")).await?.unwrap();

    b.DB.batch(vec![
        b.DB.prepare("UPDATE cards SET title = ?1 WHERE id = ?2")
            .bind(&[title.into(), id.into()])?,
        b.DB.prepare(format!("DELETE FROM todos WHERE id IN ({})"))
    ]).await?;

    Ok(())
}

#[worker::send]
pub async fn delete_card(id: &str,
    b:    Bindings,
    auth: Memory<'_, JWTPayload>
) -> Result<(), ServerError> {
    b.assert_user_is_owner_of_card(&auth.user_id, id).await?;

    b.DB.batch(vec![
        b.DB.prepare("DELETE FROM cards WHERE id = ?")
            .bind(&[id.into()])?,
        b.DB.prepare(format!("DELETE FROM todos WHERE card_id = ?"))
            .bind(&[id.into()])?,
    ]).await?;

    Ok(())
}
