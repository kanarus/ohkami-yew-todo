mod fetch;
mod hooks;

use crate::models::{Card, SignupResponse, Todo, UpdateCard};
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
                <TodoCardList />
            </Suspense>
        </>
    }
}

#[function_component]
fn TodoCardList() -> HtmlResult {
    let tokenstore = use_tokenstore();
    let user_token = use_state(|| tokenstore.get());

    let todo_cards = use_state(|| vec![]);

    if let Err(err) = &*use_future(|| {
        let tokenstore = tokenstore.clone();
        let user_token = user_token.clone();

        let todo_cards = todo_cards.clone();

        async move {
            let token = match &*user_token {
                Some(token) => Rc::clone(token),
                None => {
                    let SignupResponse { token } = Client::new(None)
                        .POST("/signup").await?.json().await?;
                    let token = Rc::new(Into::<String>::into(token));
                    {
                        tokenstore.store(&token);
                        user_token.set(Some(Rc::clone(&token)));
                    }
                    token
                },
            };

            let cards: Vec<Card> = Client::new(token)
                .GET("/api/cards").await?.json().await?;
            todo_cards.set(cards);

            Result::<(), fetch::Error>::Ok(())
        }
    })? {
        web_sys::window().unwrap().alert_with_message(&err.to_string()).unwrap();
    }

    Ok(html! {
        <>
            {user_token.as_ref().map(|token| html!{
                for todo_cards.iter().map(|card| html! {
                    <TodoCard
                        bind={Rc::new(card.clone())}
                        token={Rc::clone(token)}
                    />
                })
            })}
        </>
    })
}

#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    bind:  Rc<Card>,
    token: Rc<String>,
}

#[function_component]
pub fn TodoCard(props: &TodoCardProps) -> HtmlResult {
    let bind   = use_state(|| Rc::clone(&props.bind));
    let client = Rc::new(Client::new(Rc::clone(&props.token)));

    let onblur = Callback::from({
        let bind   = bind.clone();
        let client = client.clone();
        move |_: FocusEvent| wasm_bindgen_futures::spawn_local({
            let bind   = bind.clone();
            let client = client.clone();
            async move {
                if let Err(err) = client.PUTwith(UpdateCard {
                    title: bind.title.clone(),
                    todos: bind.todos.clone()
                }, format!("/api/cards/{}", &bind.id)).await {
                    web_sys::window().unwrap().alert_with_message(&err.to_string()).unwrap();
                }
            }
        })
    });

    Ok(html! {
        <div {onblur}
            class="bg-neutral-100 rounded-lg rounded-tr-none border border-solid border-neutral-300 shadow-lg shadow-neutral-300 p-2 m-2"
        >
            <header class="h-8 space-x-2 flex items-center">
                <div class="grow h-7">
                    <textarea
                        autocomplete="off"
                        spellcheck="false"
                        value={(&*bind.title).to_string()}
                        class="resize-none border-none_ w-full h-full outline-none bg-inherit text-neutral-800 text-lg"
                    />
                </div>
                <img
                    src="assets/icons/delete.svg"
                    class="basis-4 h-6"
                />
            </header>

            <hr class="border-neutral-400"/>

            {for bind.todos.iter().map(|todo| html! {
                <div class="flex items-center space-x-2 | border border-solid border-red-500">
                    <div class="basis-4 h-6 | border border-solid">
                        <img
                            src={if todo.as_ref().is_some_and(|todo| todo.completed) {"assets/icons/check_box.svg"} else {"assets/icons/check_box_outline_blank.svg"}}
                            class="basis-4 h-6 | border border-solid"
                        />
                    </div>
                    <div class="grow h-6 | border border-solid border-blue-500">
                        <textarea
                            autocomplete="off"
                            spellcheck="false"
                            value={todo.as_ref().map(|todo| todo.content.to_string())}
                            class={if todo.as_ref().is_some_and(|todo| todo.completed) {
                                "m-0 p-0 resize-none w-full h-full border-none_ outline-none bg-inherit text-neutral-400"
                            } else {
                                "m-0 p-0 resize-none w-full h-full border-none_ outline-none bg-inherit text-neutral-800"
                            }}
                        />
                    </div>
                </div>
            })}
        </div>
    })
}
