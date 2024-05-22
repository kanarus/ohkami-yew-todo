mod fetch;
mod components;

use fetch::Client;
use components::{CheckBoxButton, DeleteButton, TextInput};

use crate::models::{Card, SignupResponse, Todo, UpdateCard, ID};
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
    let todo_cards = use_state(|| vec![]);

    let client = match *use_future(|| Client::new())? {
        Ok(client) => client,
        Err(error) => return Ok(html!(
            <p>{format!("Failed to sign up: {error}")}</p>
        ))
    };

    // if let Err(err) = &*use_future(|| {
    //     let tokenstore = tokenstore.clone();
    //     let user_token = user_token.clone();
// 
    //     let todo_cards = todo_cards.clone();
// 
    //     async move {
    //         let token = match &*user_token {
    //             Some(token) => Rc::clone(token),
    //             None => {
    //                 let SignupResponse { token } = Client::new(None)
    //                     .POST("/signup").await?.json().await?;
    //                 let token = Rc::new(Into::<String>::into(token));
    //                 {
    //                     tokenstore.store(&token);
    //                     user_token.set(Some(Rc::clone(&token)));
    //                 }
    //                 token
    //             },
    //         };
// 
    //         let cards: Vec<Card> = Client::new(token)
    //             .GET("/api/cards").await?.json().await?;
    //         todo_cards.set(cards);
// 
    //         Result::<(), fetch::Error>::Ok(())
    //     }
    // })? {
    //     web_sys::window().unwrap().alert_with_message(&err.to_string()).unwrap();
    // }

    let handle_check_todo = Callback::from({
        let todo_cards = todo_cards.clone();
        move |index: usize| todo_cards.set({
            let mut new_cards = (&*todo_cards).clone();
            new_cards[index].
            new_cards
        })
    });

    let handle_edit_by_index = |index: usize| Callback::from({
        let cards = todo_cards.clone();

        move |editted_card: Card| cards.set({
            let mut new_cards = (&*cards).clone();
            new_cards[index] = editted_card;
            new_cards
        })
    });

    let handle_sync = Callback::from({
        let user_token = user_token.clone();
        let todo_cards = todo_cards.clone();
        
        move |_| wasm_bindgen_futures::spawn_local({
            let Card { id, title, todos } = todo_cards[index].clone();
            let Some(token) = user_token.as_ref().map(Rc::clone) else {
                return web_sys::window().unwrap().alert_with_message("You need to sign up").unwrap();
            };

            async move {
                if let Err(err) = Client::new(token).PUTwith(
                    UpdateCard { title, todos},
                    format!("/api/cards/{id}")
                ).await {
                    web_sys::window().unwrap().alert_with_message(&format!("Failed to save TODOs: {err}")).unwrap();
                }
            }
        })
    });

    let handle_delete = Callback::from({
        let id = todo_cards[index].id.clone();

        move |_| wasm_bindgen_futures::spawn_local({
            async move {}
        })
    });

    Ok(html! {
        for todo_cards.iter().enumerate().map(|(i, card)| html! {
            <TodoCard
                bind={card.clone()}
                handle_check_todo={}
                handle_edit_todo={handle_edit_by_index(i)}
                handle_sync={handle_sync}
                handle_delete={handle_delete}
            />
        })
    })
}



#[derive(Properties, PartialEq)]
struct TodoCardProps {
    title:    String,
    toolbox:  Html,
    children: Html,
}

#[function_component]
pub fn TodoCard(props: &TodoCardProps) -> Html {
    html!(
        <div class="bg-neutral-100 rounded-lg rounded-tr-none border border-solid border-neutral-300 shadow-lg shadow-neutral-300 p-2 m-2">
            <header class="h-8 space-x-2 flex items-center">
                <TextInput
                    class={"grow h-7 text-neutral-800 text-lg"}
                    value={props.title.clone()}
                    on_input={Callback::from(|_| todo!())}
                />
                {props.toolbox.clone()}
            </header>

            <hr class="border-neutral-400"/>

            {props.children.clone()}
        </div>
    )
}


/*
#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    bind:              Card,
    handle_check_todo: Callback<usize>,
    handle_edit_todo:  Callback<usize>,
    handle_delete:     Callback<()>,
    handle_sync:       Callback<()>,
}

#[function_component]
pub fn TodoCard(props: &TodoCardProps) -> Html {
    html! {
        <TodoCardLayout
            contents={TodoCardContents {
                id:      props.bind.id.clone(),
                title:   props.bind.title.clone(),
                entries: props.bind.todos.iter().map(|Todo { content, completed }| TodoEntry {
                    checked: *completed,
                    body:    content.clone(),
                }).collect()
            }}
            request_check_todo={props.handle_check_todo.clone()}
            request_edit_todo={props.handle_edit_todo.clone()}
            request_delete={props.handle_delete.clone()}
            request_sync={props.handle_sync.clone()}
        />
    }
}


#[derive(Properties, PartialEq)]
pub struct TodoCardLayoutProps {
    pub contents: TodoCardContents,

    pub request_check_todo: Callback<usize>,
    pub request_edit_todo:  Callback<usize>,
    pub request_delete:     Callback<()>,
    pub request_sync:       Callback<()>,
}
#[derive(PartialEq, Clone)]
pub struct TodoCardContents {
    pub id:      String,
    pub title:   String,
    pub entries: Vec<TodoEntry>,
}
#[derive(PartialEq, Clone)]
pub struct TodoEntry {
    pub checked: bool,
    pub body:    String,
}

#[function_component]
pub fn TodoCardLayout(props: &TodoCardLayoutProps) -> Html {
    let on_blur = props.request_sync.reform(|_: FocusEvent| ());

    let on_click_delete = props.request_delete.reform(|_: MouseEvent| ());

    let on_check_todo_by_index = |i| props.request_check_todo.reform(move |_: MouseEvent| i);

    let on_edit_todo_by_index = |i| props.request_edit_todo.reform(move |_: Event| i);

    html! {
        <div onblur={on_blur}
            class="bg-neutral-100 rounded-lg rounded-tr-none border border-solid border-neutral-300 shadow-lg shadow-neutral-300 p-2 m-2"
        >
            <header class="h-8 space-x-2 flex items-center">
                <div class="grow h-7">
                    <textarea
                        autocomplete="off"
                        spellcheck="false"
                        value={props.contents.title.clone()}
                        class="resize-none border-none_ w-full h-full outline-none bg-inherit text-neutral-800 text-lg"
                    />
                </div>
                <a onclick={on_click_delete}>
                    <img
                        src="assets/icons/delete.svg"
                        class="basis-4 h-6"
                    />
                </a>
            </header>

            <hr class="border-neutral-400"/>

            {for props.contents.entries.iter().enumerate().map(|(i, TodoEntry { checked, body })| html! {
                <div class="flex items-center space-x-2 | border border-solid border-red-500">
                    <div class="basis-4 h-6 | border border-solid">
                        <a onclick={on_check_todo_by_index(i)}>
                            <img
                                src={if *checked {"assets/icons/check_box.svg"} else {"assets/icons/check_box_outline_blank.svg"}}
                                class="basis-4 h-6 | border border-solid"
                            />
                        </a>
                    </div>
                    <div class="grow h-6 | border border-solid border-blue-500">
                        <textarea onchange={on_edit_todo_by_index(i)}
                            autocomplete="off"
                            spellcheck="false"
                            value={body.clone()}
                            class={if *checked {
                                "m-0 p-0 resize-none w-full h-full border-none_ outline-none bg-inherit text-neutral-400"
                            } else {
                                "m-0 p-0 resize-none w-full h-full border-none_ outline-none bg-inherit text-neutral-800"
                            }}
                        />
                    </div>
                </div>
            })}
        </div>
    }
}
*/
