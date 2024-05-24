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
use std::array;


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
            .bind(&array::from_fn::<_, {Card::N_TODOS}, _>(|_| (&id).into()))?,
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
        b.DB.prepare(format!(
                "SELECT card_id, content, completed_at FROM todos
                WHERE card_id IN ({})
                ORDER BY CASE card_id {} END, id DESC",
                /* reversed order to pop later from smaller card_id, id */
                ["?"; Card::N_TODOS].join(","),
                array::from_fn::<_, {Card::N_TODOS}, _>(|o| Card::N_TODOS - o)
                    .map(|rank| format!("WHEN ? THEN {rank}")).join(" ")
            ))
            .bind(&Iterator::chain(card_records.iter(), card_records.iter())
                .map(|r| (&r.id).into()).collect::<Vec<_>>()
            )?
            .all().await?.results::<Record>()?
    };

    Ok(card_records.into_iter().map(|r| {
        let todos = array::from_fn(|_| {
            let next_todo = todo_records.pop().unwrap();

            #[cfg(debug_assertions)]
            assert_eq!(next_todo.card_id, r.id, "Popped TODO has unexpected card_id");

            Todo {
                content:   next_todo.content,
                completed: next_todo.completed_at.is_some()
            }
        });

        #[cfg(debug_assertions)]
        assert_eq!(todo_records.len(), 0, "Some remaining TODO records found");

        Card {
            id:    r.id,
            title: r.title,
            todos
    }}).collect())
}

#[worker::send]
pub async fn update_card(id: &str,
    b:    Bindings,
    auth: Memory<'_, JWTPayload>,
    req:  UpdateCard,
) -> Result<(), ServerError> {
    b.assert_user_is_owner_of_card(&auth.user_id, id).await?;

    let current_title = b.DB.prepare("SELECT title FROM cards WHERE id = ?")
        .bind(&[id.into()])?.first::<String>(Some("id")).await?.unwrap();

    let current_todos = {
        #[derive(Deserialize)] struct Record {
            id:           usize,
            content:      String,
            completed_at: Option<u64>,
        }
        impl PartialEq<Todo> for Record {
            fn eq(&self, other: &Todo) -> bool {
                self.content == other.content &&
                self.completed_at.is_some() == other.completed
            }
        }
        b.DB.prepare("SELECT id, content, completed_at FROM todos WHERE card_id = ?")
            .bind(&[id.into()])?.all().await?.results::<Record>()?
    };
    
    b.DB.batch({
        let mut updates = Vec::with_capacity(1);

        if current_title != req.title {
            updates.push(
                b.DB.prepare("UPDATE cards SET title = ?1 WHERE id = ?2")
                    .bind(&[req.title.into(), id.into()])?
            )
        }

        for (current, new) in current_todos.into_iter().zip(req.todos) {
            if current != new {
                updates.push(b.DB
                    .prepare("UPDATE todos SET content = ?1, completed_at = ?2 WHERE id = ?3")
                    .bind(&[
                        (&new.content).into(),
                        new.completed.then_some(unix_timestamp() as usize).into(),
                        current.id.into()
                    ])?
                )
            }
        }

        updates
    }).await?;

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
