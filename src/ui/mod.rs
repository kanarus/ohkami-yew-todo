mod client;
mod hooks;

use crate::models::{SignupResponse, Todo};
use client::Client;
use hooks::use_token;
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use std::rc::Rc;


#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <h1 class="w-full text-center text-slate-600 underline underline-offset-8">
                {"Ohkami*Yew TODO Demo"}
            </h1>
            <Suspense>
                <Gate />
            </Suspense>
        </>
    }
}

#[function_component]
pub fn Gate() -> HtmlResult {
    let token_store = use_token();

    if let Err(err) = &*use_future(|| {
        let token_store = token_store.clone();

        async move {
            if token_store.get().is_none() {
                let SignupResponse { token } = Client::new(None)
                    .POST("/signup").await?.json().await?;
                token_store.set(&token);
            }
            Result::<(), client::Error>::Ok(())
        }
    })? {
        web_sys::window().unwrap().alert_with_message(&err.to_string()).unwrap();
    }

    Ok(html! {
        <Suspense>
            {token_store.as_ref().map(|token| html! {
                <TodoList {token} />
            })}
        </Suspense>
    })
}

#[derive(Properties, PartialEq)]
pub struct TodoListProps {
    token: Rc<String>,
}

#[function_component]
pub fn TodoList(props: &TodoListProps) -> HtmlResult {
    let todos = use_state(|| vec![]);

    if let Err(err) = &*use_future(|| {
        let todos = todos.clone();
        let token = props.token.clone();

        async move {
            let fetched_todos: Vec<Todo> = Client::new(token)
                .GET("/todos").await?.json().await?;
            todos.set(fetched_todos);
            Result::<(), client::Error>::Ok(())
        }
    })? {
        web_sys::window().unwrap().alert_with_message(&err.to_string()).unwrap();
    }

    Ok(html! {
        <div>
            {for todos.iter().map(|todo| {html! {
                <div>
                    <p>{&todo.content}</p>
                </div>
            }})}  
        </div>
    })
}
