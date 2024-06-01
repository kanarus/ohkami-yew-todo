use yew::prelude::*;
use super::atoms::{TextInput, DeleteButton};
use super::layouts::{CardLayout, TodoLayout};
use crate::models::Card;


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


#[derive(Properties, PartialEq)]
pub struct PlusCardProps {
    pub on_click: Callback<()>,
}

#[function_component]
pub fn PlusCard(props: &PlusCardProps) -> Html {
    html!(
        <div
            class="
                bg-slate-100
                cursor-pointer
                rounded-xl rounded-tr-none
                w-72 min-w-72
                flex
            "
            onclick={props.on_click.reform(|_| ())}
        >
            <p class="
                m-auto
                text-2xl text-neutral-800
            ">{"＋"}</p>
        </div>
    )
}
