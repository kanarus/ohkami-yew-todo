mod fetch;
mod hooks;

use crate::models::{SignupResponse, Todo};
use fetch::Client;
use hooks::use_tokenstore;
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use std::rc::Rc;


#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <header>
                <h1 class="w-full text-center text-neutral-800 underline underline-offset-8">
                    {"Ohkami*Yew TODO Demo"}
                </h1>
            </header>
            <Suspense>
                <TodoDemo />
            </Suspense>
        </>
    }
}

#[function_component]
pub fn TodoDemo() -> HtmlResult {
    let tokenstore = use_tokenstore();
    let user_token = use_state(|| tokenstore.get());

    if let Err(err) = &*use_future(|| {
        let tokenstore = tokenstore.clone();
        let user_token = user_token.clone();

        async move {
            if user_token.is_none() {
                let SignupResponse { token } = Client::new(None)
                    .POST("/signup").await?.json().await?;
                tokenstore.store(&token);
                user_token.set(Some(Rc::new(token.into())));
            }
            Result::<(), fetch::Error>::Ok(())
        }
    })? {
        web_sys::window().unwrap().alert_with_message(&err.to_string()).unwrap();
    }

    Ok(html! {
        <>
            {user_token.as_ref().map(|token| html! {
                <Suspense>
                    <TodoCard {token} />
                </Suspense>
            })}
        </>
    })
}

#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    token: Rc<String>,
}

#[function_component]
pub fn TodoCard(props: &TodoCardProps) -> HtmlResult {
    let todos  = use_state(|| vec![]);
    let client = Rc::new(Client::new(props.token.clone()));

    if let Err(err) = &*use_future(|| {
        let todos  = todos.clone();
        let client = client.clone();

        async move {
            let fetched_todos: Vec<Todo> = client
                .GET("/api/todos").await?.json().await?;
            todos.set(fetched_todos);
            Result::<(), fetch::Error>::Ok(())
        }
    })? {
        web_sys::window().unwrap().alert_with_message(&err.to_string()).unwrap();
    }

    let sample_todos = vec![
        Todo {
            id: String::new(),
            tags: vec![],
            content: String::from("Sample ToDo"),
            completed: false,
        },
        Todo {
            id: String::new(),
            tags: vec![],
            content: String::from("This is second sample ToDo, whitch is completed."),
            completed: true,
        },
        Todo {
            id: String::new(),
            tags: vec![],
            content: String::from("ゴミ捨て・荷物回収"),
            completed: true,
        },
    ];

    Ok(html! {
        <div class="bg-neutral-100 rounded-lg rounded-tr-none border border-solid border-neutral-300 shadow-lg shadow-neutral-300 p-2 m-2">
            <header class="h-8 space-x-2 flex items-center">
                <input
                    autocomplete="off"
                    value={"さんぷるとぅーどぅー"}
                    class="grow border-none bg-inherit text-neutral-800 text-lg"
                />
                <img
                    src="assets/icons/delete.svg"
                    class="basis-4 h-6"
                />
            </header>

            <hr class="border-neutral-400"/>

            {for sample_todos.iter().map(|todo| html! {
                <div class="flex items-center space-x-2">
                    <img
                        src={if todo.completed {"assets/icons/check_box.svg"} else {"assets/icons/check_box_outline_blank.svg"}}
                        class="basis-4 h-6"
                    />
                    <input
                        autocomplete="off"
                        value={todo.content.clone()}
                        class={if todo.completed {
                            "grow border-none bg-inherit text-neutral-400"
                        } else {
                            "grow border-none bg-inherit text-neutral-800"
                        }}
                    />
                </div>
            })}
        </div>
    })
}
