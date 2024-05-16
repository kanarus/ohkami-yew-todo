mod server;
mod models;

use server::{complete_todo, create_todo, list_todos, signup, update_todo};
use server::jwt;
use ohkami::prelude::*;


#[ohkami::bindings]
struct Bindings;

#[ohkami::worker]
async fn my_worker() -> Ohkami {
    console_error_panic_hook::set_once();

    Ohkami::new((
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
