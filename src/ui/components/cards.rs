use yew::prelude::*;
use super::atoms::{DeleteButton, UploadButton};
use super::layouts::{CardLayout, TodoLayout};
use crate::models::{Card, CreateCardRequest, Todo};



#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    pub bind:    Card,
    pub handler: TodoCardHandler,
}

#[derive(PartialEq)]
pub struct TodoCardHandler {
    pub on_request_save:  Callback<()>,
    pub on_click_delete:  Callback<()>,
    pub on_edit_title:    Callback<String>,
    pub on_check_todo_by: [Callback<()>; Card::N_TODOS],
    pub on_edit_todo_by:  [Callback<String>; Card::N_TODOS],
}

#[function_component]
pub fn TodoCard(props: &TodoCardProps) -> Html {
    html!(
        <CardLayout
            title={props.bind.title.clone()}
            on_edit_title={props.handler.on_edit_title.clone()}
            on_blur={props.handler.on_request_save.clone()}
            toolbox={html!(
                <DeleteButton
                    on_click={props.handler.on_click_delete.clone()}
                />
            )}
            contents={html!(
                <TodoLayout
                    todos={props.bind.todos.clone()}
                    on_check_todo={props.handler.on_check_todo_by.clone()}
                    on_edit_todo={props.handler.on_edit_todo_by.clone()}
                />
            )}
        />
    )
}


#[derive(Properties, PartialEq)]
pub struct PlaceholderCardProps {
    pub handler: PlaceholderCardHandler,
}

#[derive(PartialEq, Clone)]
pub struct PlaceholderCardHandler {
    pub on_initial_input: Callback<UseStateHandle<CreateCardRequest>>,
}

#[function_component]
pub fn PlaceholderCard(props: &PlaceholderCardProps) -> Html {
    let input = use_state(CreateCardRequest::empty);

    html!(
        <CardLayout
            title={input.title.clone()}
            on_edit_title={Callback::from({
                let input = input.clone();
                move |value| input.set({
                    let mut input = (&*input).clone();
                    input.title = value;
                    input
                })
            })}
            toolbox={html!(
                <UploadButton
                    on_click={(!input.is_empty()).then_some({
                        let (input, handler) = (input.clone(), props.handler.clone());
                        handler.on_initial_input.reform(move |_| input.clone())
                    })}
                />
            )}
            contents={html!(
                <TodoLayout
                    checkable={false}
                    todos={input.todos.clone().map(|content| Todo { content, completed: false })}
                    on_edit_todo={std::array::from_fn(|i| Callback::from({
                        let input = input.clone();
                        move |value| input.set({
                            let mut input = (&*input).clone();
                            input.todos[i] = value;
                            input
                        })
                    }))}
                />
            )}
        />
    )
}


#[function_component]
pub fn FrontCoverCard() -> Html {
    html!(
        <CardLayout
            title={String::from("Note")}
            toolbox={/* empty */}
            contents={html!(
                <ul class="m-0">
                    <li>{"デモなので、適当に永久トークンを発行してlocalStorageに保存し、それをもってユーザーを識別しています。"}</li>
                    <li>{"念のため、知られてはいけない情報は入力しないことをおすすめします。"}</li>
                </ul>
            )}
        />
    )
}
