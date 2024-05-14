mod fangs;
mod models;

use fangs::{jwt, JWTPayload};
use models::{Tag, Todo};

use ohkami::prelude::*;
use ohkami::serde::Deserialize;
use ohkami::Memory;


#[ohkami::bindings]
struct Bindings;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Error in worker: {0}")]
    Worker(#[from] worker::Error),
}
impl IntoResponse for Error {
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
            "/list".GET(list_todos),
        ))),
    ))
}


#[worker::send]
async fn list_todos(b: Bindings, auth: Memory<'_, JWTPayload>) -> Result<Vec<Todo>, Error> {
    let user_id = &auth.user_id;

    let todo_records = {
        #[derive(Deserialize)] struct Record {
            id:             String,
            content:        String,
            completed_at:   Option<u64>,
            tag_ids_csv:    String,
            tag_labels_csv: String,
        }
        b.DB.prepare("
        SELECT
            id,
            content,
            completed_at,
            group_concat(tags.id) AS tag_ids_csv,
            group_concat(tags.label) AS tag_labels_csv
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
            r.tag_labels_csv.split(',').map(|label| label.to_string())
        ).map(|(id, label)| Tag { id, label }).collect(),
    }).collect())
}
