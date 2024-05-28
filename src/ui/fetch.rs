use ohkami::serde::Serialize;
use crate::models::SignupResponse;

pub use reqwest::Error;


pub struct Client(reqwest::Client);

impl Client {
    const TOKEN_STORAGE_KEY: &'static str = "ohkami-yew-todo-demo-token";

    const ORIGIN: &'static str = {
        #[cfg(debug_assertions)] {"http://localhost:8787"}
        #[cfg(not(debug_assertions))] {"https://ohkami-yew-todo.kanarus.workers.dev"}
    };

    pub async fn new() -> Result<Self, Error> {
        let token = {
            let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

            match local_storage.get(Self::TOKEN_STORAGE_KEY).unwrap() {
                Some(token) => token,
                None => {
                    let SignupResponse { token } = reqwest::Client::new()
                        .post(format!("{}/signup", Self::ORIGIN)).send().await?.json().await?;
                    local_storage.set(Self::TOKEN_STORAGE_KEY, &token).unwrap();
                    token.into()
                }
            }
        };

        let client = reqwest::ClientBuilder::new()
            .default_headers(FromIterator::from_iter([(
                "Authorization".try_into().unwrap(),
                format!("Bearer {token}").try_into().unwrap()
            )]))
            .build().unwrap();

        Ok(Self(client))
    }
}

macro_rules! call {
    ( $( $method:ident & $with_body_method:ident ),* ) => {
        #[allow(non_snake_case, unused)]
        impl Client {$(
            pub async fn $method(&self,
                path: impl AsRef<str>
            ) -> Result<reqwest::Response, Error> {
                self.0.request(
                    reqwest::Method::$method,
                    format!("{}{}", Self::ORIGIN, path.as_ref())
                ).send().await
            }

            pub async fn $with_body_method<Body: Serialize>(&self,
                body: Body,
                path: impl AsRef<str>
            ) -> Result<reqwest::Response, Error> {
                self.0.request(
                    reqwest::Method::$method,
                    format!("{}{}", Self::ORIGIN, path.as_ref())
                ).json(&body).send().await
            }
        )*}
    };
} call! {
    GET & GETwith,
    PUT & PUTwith,
    POST & POSTwith,
    PATCH & PATCHwith,
    DELETE & DELETEwith
}
