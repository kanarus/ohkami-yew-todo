mod fangs;
mod models;
mod statements;

use fangs::{jwt, JWTPayload};
use models::{CreateTodo, Tag, Todo};

use ohkami::prelude::*;
use ohkami::typed::status;
use ohkami::serde::Deserialize;
use ohkami::Memory;


#[ohkami::bindings]
struct Bindings;

#[derive(Debug, thiserror::Error)]
enum ServerError {
    #[error("Error in worker: {0}")]
    Worker(#[from] worker::Error),
}
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        worker::console_error!("{self}");

        match self {
            Self::Worker(_) => Response::InternalServerError(),
        }
    }
}

#[ohkami::worker]
async fn my_worker() -> Ohkami {
    console_error_panic_hook::set_once();

    Ohkami::new((
        /* `dist` is served by `--assets dist` option passed to `dev` script in package.json */

        "/api".By(Ohkami::with(jwt(), (
            "/todos"
                .GET(list_todos)
                .POST(create_todo),
        ))),
    ))
}

#[worker::send]
async fn create_todo(
    b:    Bindings,
    auth: Memory<'_, JWTPayload>,
    req:  CreateTodo<'_>,
) -> Result<status::Created<Todo>, ServerError> {
    let user_id = &auth.user_id;

    let id = web_sys::window().unwrap().crypto().unwrap().random_uuid();
    let tags = b.get_or_create_tags_by_names(&req.tags).await?;
    
    b.DB.prepare("
    INSERT INTO todos (
        id, user_id, content
    ) VALUES (
        ?1, ?2,      ?3
    )")
        .bind(&[(&id).into(), user_id.into(), req.content.into()])?
        .run().await?;

    b.DB.prepare(format!("
    INSERT INTO todo_tags (
        todo_id, tag_id
    ) VALUES {}",
    vec!["(?,?)"; tags.len()].join(",")))
        .bind(&tags.iter()
            .map(|tag| [(&id).into(), tag.id.into()])
            .flatten().collect::<Vec<_>>()
        )?
        .run().await?;

    Ok(status::Created(Todo {
        id,
        tags,
        content:   req.content.into(),
        completed: false,
    }))
}

#[worker::send]
async fn list_todos(
    b:    Bindings,
    auth: Memory<'_, JWTPayload>,
) -> Result<Vec<Todo>, ServerError> {
    let user_id = &auth.user_id;

    let todo_records = {
        #[derive(Deserialize)] struct Record {
            id:            String,
            content:       String,
            completed_at:  Option<u64>,
            tag_ids_csv:   String,
            tag_names_csv: String,
        }
        b.DB.prepare("
        SELECT
            id,
            content,
            completed_at,
            group_concat(tags.id) AS tag_ids_csv,
            group_concat(tags.name) AS tag_names_csv
        FROM todos
        JOIN todo_tags ON todo_tags.todo_id = todos.id
        JOIN tags      ON tags.id = todo_tags.tag_id
        WHERE user_id = ?1
        GROUP BY todos.id")
            .bind(&[user_id.into()])?
            .all().await?.results::<Record>()?
    };

    Ok(todo_records.into_iter().map(|r| Todo {
        id:        r.id,
        content:   r.content,
        completed: r.completed_at.is_some(),
        tags: Iterator::zip(
            r.tag_ids_csv.split(',').map(|id| id.parse().unwrap()),
            r.tag_names_csv.split(',').map(|name| name.to_string())
        ).map(|(id, name)| Tag { id, name }).collect(),
    }).collect())
}
