mod api;
mod models;

use api::{signup, list_cards, create_card, update_card};
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
            "/cards"
                .GET(list_cards)
                .POST(create_card),
            "/cards/:id"
                .PUT(update_card),
        ))),
    ))
}
