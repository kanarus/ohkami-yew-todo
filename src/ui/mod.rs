mod client;
mod hooks;

use crate::models::{SignupResponse, Todo};
use client::Client;
use hooks::use_tokenstore;
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
                <TODODemo />
            </Suspense>
        </>
    }
}

#[function_component]
pub fn TODODemo() -> HtmlResult {
    let tokenstore = use_tokenstore();
    let user_token = use_state(|| tokenstore.get());

    if let Err(err) = &*use_future(|| {
        let tokenstore = tokenstore.clone();
        let user_token = user_token.clone();

        async move {
            if user_token.is_none() {
                let SignupResponse { token } = Client::new(None)
                    .POST("/signup").await?.json().await?;
                tokenstore.set(&token);
                user_token.set(Some(Rc::new(token.into())));
            }
            Result::<(), client::Error>::Ok(())
        }
    })? {
        web_sys::window().unwrap().alert_with_message(&err.to_string()).unwrap();
    }

    Ok(html! {
        <>
            {user_token.as_ref().map(|token| html! {
                <Suspense>
                    <TodoList {token} />
                </Suspense>
            })}
        </>
    })
}

#[derive(Properties, PartialEq)]
pub struct TodoListProps {
    token: Rc<String>,
}

#[function_component]
pub fn TodoList(props: &TodoListProps) -> HtmlResult {
    let todos  = use_state(|| vec![]);
    let client = Rc::new(Client::new(props.token.clone()));

    if let Err(err) = &*use_future(|| {
        let todos  = todos.clone();
        let client = client.clone();

        async move {
            let fetched_todos: Vec<Todo> = client
                .GET("/api/todos").await?.json().await?;
            todos.set(fetched_todos);
            Result::<(), client::Error>::Ok(())
        }
    })? {
        web_sys::window().unwrap().alert_with_message(&err.to_string()).unwrap();
    }

    Ok(html! {
        <div>
            <div>
                <span>{"Sample Todo"}</span>
                <span>{&false}</span>
            </div>
            {for todos.iter().map(|todo| {html! {
                <div>
                    <span>{&todo.content}</span>
                    <span>{&todo.completed}</span>
                </div>
            }})}
        </div>
    })
}
