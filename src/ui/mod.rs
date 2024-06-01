mod utils;
mod fetch;
mod components;

use fetch::Client;
use utils::{set_state, report_error};
use components::{TodoCard, TodoCardHandler, FrontCoverCard, PlusCard};

use crate::models::{Card, CreateCardRequest, CreateCardResponse, Todo, UpdateCard};
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use std::rc::Rc;


#[function_component]
pub fn App() -> Html {
    html! (
        <main class="h-full flex flex-col">
            <header class="basis-12 mt-12">
                <h1 class="m-0 w-full h-12 text-center text-neutral-800 underline underline-offset-8">
                    {"Ohkami×Yew TODO Demo"}
                </h1>
            </header>
            <div class="grow flex items-center">
                <div class="overflow-hidden">
                    <Suspense fallback={html!(<p class="w-screen text-center">{"Loading..."}</p>)}>
                        <Main />
                    </Suspense>
                </div>
            </div>
        </main>
    )
}

#[function_component]
fn Main() -> HtmlResult {
    let client = match &*use_future(|| async {Client::new().await.map(Rc::new)})? {
        Ok(client) => client.clone(),
        Err(err)   => {
            report_error(format!("Can't perform sign up: {err}"));
            return Ok(html!(/* empty */))
        }
    };

    Ok(html!(
        <Suspense fallback={html!(<p class="w-screen text-center">{"Loading..."}</p>)}>
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
            let fetched_cards: Vec<Card> = client.GET("/api/cards").await?.json().await?;
            cards.set(fetched_cards);
            Result::<(), fetch::Error>::Ok(())
        }
    })?.is_err() {
        report_error("Failed to fetch your TODOs");
    }

    let todo_handlers = (0..cards.len()).map(|i| TodoCardHandler {
        on_click_delete: Callback::from({
            let (client, cards) = (client.clone(), cards.clone());
            move |_| wasm_bindgen_futures::spawn_local({
                let (client, cards) = (client.clone(), cards.clone());
                async move {
                    let Card { id, .. } = cards[i].clone();
                    match client.DELETE(format!("/api/cards/{id}")).await {
                        Err(_) => report_error("Failed to delete this TODO"),
                        Ok(_)  => set_state(&cards, |cs| {cs.remove(i);})
                    }
                }
            })
        }),
        on_edit_title: Callback::from({
            let (client, cards) = (client.clone(), cards.clone());
            move |new_title: String| wasm_bindgen_futures::spawn_local({
                let (client, cards) = (client.clone(), cards.clone());

                set_state(&cards, |cs| cs[i].title = new_title.clone());

                async move {
                    let Card { id, todos, title:_ } = cards[i].clone();
                    if let Err(err) = client.PUTwith(UpdateCard {
                        todos,
                        title: new_title
                    }, format!("/api/cards/{id}")).await {
                        report_error(&format!("Failed to update title: {err}"));
                        set_state(&cards, |_| (/* stay */));
                    }
                }
            })
        }),
        on_check_todo_by: std::array::from_fn(|j| Callback::from({
            let (client, cards) = (client.clone(), cards.clone());
            move |_| wasm_bindgen_futures::spawn_local({
                let (client, cards) = (client.clone(), cards.clone());

                let new_todos: [_; Card::N_TODOS] = std::array::from_fn(|k| if k == j {
                    let mut current = cards[i].todos[k].clone();
                    current.completed = !current.completed;
                    current
                } else {
                    cards[i].todos[k].clone()
                });

                set_state(&cards, |cs| cs[i].todos = new_todos.clone());

                async move {
                    let Card { id, title, todos:_ } = cards[i].clone();
                    if let Err(err) = client.PUTwith(UpdateCard {
                        title,
                        todos: new_todos
                    }, format!("/api/cards/{id}")).await {
                        report_error(&format!("Failed to update TODO: {err}"));
                        set_state(&cards, |_| (/* stay */));
                    }
                }
            })
        })),
        on_edit_todo_by: std::array::from_fn(|j| Callback::from({
            let (client, cards) = (client.clone(), cards.clone());
            move |new_content: String| wasm_bindgen_futures::spawn_local({
                let (client, cards) = (client.clone(), cards.clone());
                
                let new_todos: [_; Card::N_TODOS] = std::array::from_fn(|k| if k == j {
                    let mut current = cards[i].todos[k].clone();
                    current.content = new_content.clone();
                    current
                } else {
                    cards[i].todos[k].clone()
                });

                set_state(&cards, |cs| cs[i].todos = new_todos.clone());

                async move {
                    let Card { id, title, todos:_ } = cards[i].clone();
                    if let Err(err) = client.PUTwith(UpdateCard {
                        title,
                        todos: new_todos
                    }, format!("/api/cards/{id}")).await {
                        report_error(&format!("Failed to update TODO: {err}"));
                        set_state(&cards, |_| (/* stay */));
                    }
                }
            })
        })),
    });

    let handle_click_plus = Callback::from({
        let (client, cards) = (client.clone(), cards.clone());
        move |_| wasm_bindgen_futures::spawn_local({
            let (client, cards) = (client.clone(), cards.clone());

            set_state(&cards, |cs| cs.push(Card {
                id:    String::new(),
                title: String::new(),
                todos: std::array::from_fn(|_| Todo {
                    content:   String::new(),
                    completed: false,
                }),
            }));

            async move {
                match async {Result::<_, fetch::Error>::Ok(client
                    .POSTwith(CreateCardRequest::empty(), "/api/cards").await?
                    .json().await?
                )}.await {
                    Ok(CreateCardResponse { id }) => {
                        set_state(&cards, |cs| cs.push(Card {
                            id,
                            title: String::new(),
                            todos: std::array::from_fn(|_| Todo {
                                content:   String::new(),
                                completed: false,
                            }),
                        }))
                    }
                    Err(_) => {
                        report_error("Failed to create TODO card");
                        set_state(&cards, |_| (/* stay */));
                    }
                }
            }
        })
    });

    Ok(html! {
        <div class="
            m-0 px-6 space-x-4
            overflow-x-scroll overflow-y-hidden
            flex
        ">
            <FrontCoverCard />
            {for cards.iter().cloned().zip(todo_handlers).map(|(card, handler)| html!(
                <TodoCard bind={card} handler={handler} />
            ))}
            <PlusCard on_click={handle_click_plus} />
        </div>
    })
}
