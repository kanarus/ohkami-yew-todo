use yew::prelude::*;
use super::atoms::{TextInput, DeleteButton, UploadButton};
use super::layouts::{CardLayout, TodoLayout};
use super::super::utils::set_state;
use crate::models::{Card, CreateCardRequest, Todo};



#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    pub bind:    Card,
    pub handler: TodoCardHandler,
}

#[derive(PartialEq)]
pub struct TodoCardHandler {
    pub on_click_delete:  Callback<()>,
    pub on_edit_title:    Callback<String>,
    pub on_check_todo_by: [Callback<()>; Card::N_TODOS],
    pub on_edit_todo_by:  [Callback<String>; Card::N_TODOS],
}

#[function_component]
pub fn TodoCard(props: &TodoCardProps) -> Html {
    html!(
        <CardLayout
            title={html!(
                <TextInput
                    is_title={true}
                    value={props.bind.title.clone()}
                    on_change={props.handler.on_edit_title.clone()}
                />
            )}
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
    pub on_request_create: Callback<UseStateHandle<CreateCardRequest>>,
}

#[function_component]
pub fn PlaceholderCard(props: &PlaceholderCardProps) -> Html {
    let input = use_state(CreateCardRequest::empty);

    html!(
        <div onkeydown={(!input.is_empty()).then_some({
            let (input, handler) = (input.clone(), props.handler.clone());
            move |e: KeyboardEvent| if e.ctrl_key() && e.key() == "Enter" {
                handler.on_request_create.emit(input.clone())
            }
        })}>
            <CardLayout
                title={html!(
                    <TextInput
                        is_title={true}
                        value={input.title.clone()}
                        on_input={Callback::from({
                            let input = input.clone();
                            move |value| set_state(&input, |ip| ip.title = value)
                        })}
                    />
                )}
                toolbox={html!(
                    <UploadButton
                        on_click={(!input.is_empty()).then_some({
                            let (input, handler) = (input.clone(), props.handler.clone());
                            handler.on_request_create.reform(move |_| input.clone())
                        })}
                    />
                )}
                contents={html!(
                    <TodoLayout
                        checkable={false}
                        todos={input.todos.clone().map(|content| Todo { content, completed: false })}
                        on_edit_todo={std::array::from_fn(|i| Callback::from({
                            let input = input.clone();
                            move |value| set_state(&input, |ip| ip.todos[i] = value)
                        }))}
                    />
                )}
            />
        </div>
    )
}


#[function_component]
pub fn FrontCoverCard() -> Html {
    html!(
        <CardLayout
            title={html!(
                <TextInput
                    is_title={true}
                    value={String::from("Note")}
                />
            )}
            toolbox={/* empty */}
            contents={html!(
                <ul class="m-0">
                    <li>{"自動的に永久トークンを発行してlocalStorageに保存し、それをもってユーザーを識別しています。"}</li>
                    <li>{"念のため、知られてはいけない情報は入力しないことをおすすめします。"}</li>
                    <li>
                        {"repository: "}
                        <a
                            href="https://github.com/kana-rus/ohkami-yew-todo"
                            rel="noopener noreferrer"
                            target="_blank"
                        >
                            {"https://github.com/kana-rus/ohkami-yew-todo"}
                        </a>
                    </li>
                </ul>
            )}
        />
    )
}
