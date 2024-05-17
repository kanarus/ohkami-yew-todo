mod api;
mod models;

use api::{complete_todo, create_todo, list_todos, signup, update_todo};
use api::jwt;
use ohkami::prelude::*;


#[ohkami::bindings]
struct Bindings;

#[ohkami::worker]
async fn my_worker() -> Ohkami {
    console_error_panic_hook::set_once();

    let fangs = {
        #[cfg(debug_assertions)]
        ohkami::builtin::fang::CORS::new("http://127.0.0.1:8080")
    };

    Ohkami::with(fangs, (
        /* `dist` is served by `--assets dist` of `dev` script in package.json */

        "/signup"
            .POST(signup),

        "/api".By(Ohkami::with(jwt::fang(), (
            "/todos"
                .GET(list_todos)
                .POST(create_todo),
            "/todos/:id"
                .PUT(update_todo)
                .PATCH(complete_todo)
        ))),
    ))
}
