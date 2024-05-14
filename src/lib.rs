mod render;
use render::HTML;

use ohkami::prelude::*;
use ohkami::serde::{Serialize, Deserialize};
use ohkami::{typed::Payload, builtin::payload::JSON};
use ohkami::builtin::{fang::JWT, item::JWTToken};
use ohkami::utils::unix_timestamp;


#[ohkami::bindings]
struct Bindings;

type ID = String;

#[derive(Serialize, Deserialize)]
struct JWTPayload {
    user_id: ID,
    iat:     u64,
}

fn jwt() -> JWT<JWTPayload> {
    JWT::default(Bindings::JWT_SECRET_KEY())
}

#[ohkami::worker]
async fn my_worker() -> Ohkami {
    console_error_panic_hook::set_once();

    Ohkami::new((
        "/signin".GET(signin),
        "/api".By(Ohkami::with(jwt(),
            "/".GET(index)
        )),
    ))
}


async fn index() -> HTML {
    render!(
        <h1>{"TODO!"}</h1>
    )
}

#[Payload(JSON/S)]
pub struct SigninResponse {
    pub token: JWTToken,
}

async fn signin() -> SigninResponse {
    SigninResponse {
        token: jwt().issue(JWTPayload {
            user_id: String::from("42"),
            iat:     unix_timestamp(),
        })
    }
}
