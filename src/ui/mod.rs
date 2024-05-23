mod fetch;
mod components;

use fetch::Client;
use components::{CheckBoxButton, DeleteButton, TextInput};

use crate::models::{Card, UpdateCard};
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
            <main>
                <Suspense fallback={html!(<p>{"Loading..."}</p>)}>
                    <Main />
                </Suspense>
            </main>
        </>
    }
}

#[function_component]
fn Main() -> HtmlResult {
    let Ok(ref client) = *use_future(|| async {Client::new().await.map(Rc::new)})? else {
        return Ok(html!(<p>{format!("Can't perform sign up")}</p>))
    };

    Ok(html!(
        <Suspense fallback={html!(<p>{"Loading..."}</p>)}>
            <TodoCardList client={client.clone()}/>
        </Suspense>
    ))
}


#[derive(Properties)]
struct TodoCardListProps {
    client: Rc<Client>,
}
impl PartialEq for TodoCardListProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.client, &other.client)
    }
}

#[function_component]
fn TodoCardList(TodoCardListProps { client }: &TodoCardListProps) -> HtmlResult {
    let cards = use_state(|| vec![]);

    if use_future(|| {
        let (client, cards) = (client.clone(), cards.clone());
        async move {
            let fetched_cards: Vec<Card> = client
                .GET("/api/cards").await?.json().await?;
            cards.set(fetched_cards);
            Result::<(), fetch::Error>::Ok(())
        }
    })?.is_err() {
        web_sys::window().unwrap().alert_with_message("Failed to fetch your TODOs").unwrap();
    }

    let handlers = (0..cards.len()).map(|i| TodoCardHandler {
        on_request_save: Callback::from({
            let (client, cards) = (client.clone(), cards.clone());
            move |_| wasm_bindgen_futures::spawn_local({
                let (client, cards) = (client.clone(), cards.clone());
                async move {
                    let Card { id, title, todos } = cards[i].clone();
                    if client.PUTwith(UpdateCard { title, todos }, format!("/api/cards/{id}")).await.is_err() {
                        web_sys::window().unwrap().alert_with_message("Failed to save this update").unwrap();
                    }
                }
            })
        }),
        on_click_delete: Callback::from({
            let (client, cards) = (client.clone(), cards.clone());
            move |_| wasm_bindgen_futures::spawn_local({
                let (client, cards) = (client.clone(), cards.clone());
                async move {
                    let Card { id, .. } = cards[i].clone();
                    match client.DELETE(format!("/api/cards/{id}")).await {
                        Err(_) => web_sys::window().unwrap().alert_with_message("Failed to delete this TODO").unwrap(),
                        Ok(_)  => cards.set({
                            let mut new_cards = (&*cards).clone();
                            new_cards.remove(i);
                            new_cards
                        }),
                    }
                }
            })
        }),
        on_edit_title: Callback::from({
            let cards = cards.clone();
            move |new_title| {
                cards.set({
                    let mut new_cards = (&*cards).clone();
                    new_cards[i].title = new_title;
                    new_cards
                })
            }
        }),
        on_check_todo_by: std::array::from_fn(|j| Callback::from({
            let cards = cards.clone();
            move |_| cards.set({
                let mut new_cards = (&*cards).clone();
                new_cards[i].todos[j].completed = true;
                new_cards
            })
        })),
        on_edit_todo_by: std::array::from_fn(|j| Callback::from({
            let cards = cards.clone();
            move |new_content| cards.set({
                let mut new_cards = (&*cards).clone();
                new_cards[i].todos[j].content = new_content;
                new_cards
            })
        })),
    });

    Ok(html! {
        for cards.iter().cloned().zip(handlers).map(|(card, handler)| html! {
            <TodoCard
                bind={card}
                handler={handler}
            />
        })
    })
}


#[derive(Properties, PartialEq)]
struct TodoCardProps {
    bind:    Card,
    handler: TodoCardHandler,
}

#[derive(PartialEq)]
struct TodoCardHandler {
    on_request_save:  Callback<()>,
    on_click_delete:  Callback<()>,
    on_edit_title:    Callback<String>,
    on_check_todo_by: [Callback<()>; Card::N_TODOS],
    on_edit_todo_by:  [Callback<String>; Card::N_TODOS],
}

#[function_component]
fn TodoCard(props: &TodoCardProps) -> Html {
    html!(
        <div
            class="bg-neutral-100 rounded-lg rounded-tr-none border border-solid border-neutral-300 shadow-lg shadow-neutral-300 p-2 m-2"
            onblur={props.handler.on_request_save.reform(|_: FocusEvent| ())}
        >
            <header class="h-8 space-x-2 flex items-center">
                <TextInput
                    class="grow h-7 text-neutral-800 text-lg"
                    value={props.bind.title.clone()}
                    on_input={props.handler.on_edit_title.clone()}
                />
                <DeleteButton
                    class="basis-4 h-6"
                    on_click={props.handler.on_click_delete.clone()}
                />
            </header>

            <hr class="border-neutral-400"/>

            <ul>{for props.bind.todos.iter().enumerate().map(|(i, todo)| html!(
                <li class="list-none flex items-center space-x-2">
                    <div class={if todo.completed {"text-neutral-400"} else {"text-neutral-800"}}>
                        <CheckBoxButton
                            class="basis-4 h-6"
                            checked={todo.completed}
                            on_click={props.handler.on_check_todo_by[i].clone()}
                        />
                        <TextInput
                            class="grow h-6 m-0 p-0"
                            value={todo.content.clone()}
                            on_input={props.handler.on_edit_todo_by[i].clone()}
                        />
                    </div>
                </li>
            ))}</ul>
        </div>
    )
}
