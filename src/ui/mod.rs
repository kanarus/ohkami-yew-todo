mod fetch;
mod components;

use fetch::Client;
use components::{TodoCard, TodoCardHandler, PlaceholderCard, PlaceholderCardHandler};

use crate::models::{Card, CreateCardRequest, CreateCardResponse, Todo, UpdateCard};
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

    let todo_handlers = (0..cards.len()).map(|i| TodoCardHandler {
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

    let placeholder_handler = PlaceholderCardHandler {
        on_initial_input: Callback::from({
            let (client, cards) = (client.clone(), cards.clone());
            move |input: UseStateHandle<CreateCardRequest>| wasm_bindgen_futures::spawn_local({
                let (client, cards) = (client.clone(), cards.clone());

                let init: CreateCardRequest = (&*input).clone();

                input.set(CreateCardRequest::empty());
                cards.set({let mut cards = (&*cards).clone();
                    cards.push(Card {
                        id:    String::new(),
                        title: input.title.clone(),
                        todos: input.todos.clone().map(|content| Todo {
                            content,
                            completed: false,
                        }),
                    });
                cards});

                async move {
                    match async {Result::<_, fetch::Error>::Ok(client
                        .POSTwith(init, "/api/cards").await?
                        .json().await?
                    )}.await {
                        Ok(CreateCardResponse { id }) => {
                            cards.set({let mut cards = (&*cards).clone();
                                cards.last_mut().unwrap().id = id;
                            cards});
                        }
                        Err(_) => {
                            cards.set({let mut cards = (&*cards).clone();
                                let _ = cards.pop();
                            cards});

                            web_sys::window().unwrap().alert_with_message("Failed to create TODO card").unwrap();
                        }
                    }
                }
            })
        }),
    };

    Ok(html! {
        <div class="
            relative
            m-0
            overflow-x-scroll
            flex
        ">
            {for cards.iter().cloned().zip(todo_handlers).map(|(card, handler)| html! {
                <TodoCard
                    bind={card}
                    handler={handler}
                />
            })}
            <PlaceholderCard
                handler={placeholder_handler}
            />
        </div>
    })
}
