use yew::prelude::*;
use crate::models::{Card, Todo};
use super::atoms::{TextInput, CheckBoxButton};


#[derive(Properties, PartialEq)]
pub struct CardLayoutProps {
    pub title:    Html,
    pub toolbox:  Html,
    pub contents: Html,
}

#[function_component]
pub fn CardLayout(props: &CardLayoutProps) -> Html {
    html!(
        <div
            class="
                bg-neutral-100
                rounded-xl rounded-tr-none
                border border-solid border-neutral-300
                shadow-lg shadow-neutral-300
                w-72 min-w-72 h-[374px]
                p-4
            "
        >
            <header class="h-7 space-x-2 flex items-center">
                <div class="grow h-7">
                    {props.title.clone()}
                </div>
                <div class="basis-4 h-6">
                    {props.toolbox.clone()}
                </div>
            </header>

            <hr class="border-neutral-400 my-4" />

            {props.contents.clone()}
        </div>
    )
}


#[derive(Properties, PartialEq)]
pub struct TodoLayoutProps {
    pub todos: [Todo; Card::N_TODOS],

    #[prop_or(true)]
    pub checkable:     bool,
    #[prop_or_else(|| std::array::from_fn(|_| Callback::noop()))]
    pub on_check_todo: [Callback<()>; Card::N_TODOS],
    #[prop_or_else(|| std::array::from_fn(|_| Callback::noop()))]
    pub on_edit_todo:  [Callback<String>; Card::N_TODOS],
}

#[function_component]
pub fn TodoLayout(props: &TodoLayoutProps) -> Html {
    html!(
        <ul class="m-0 p-0 space-y-2">
            {for props.todos.iter().enumerate().map(|(i, todo)| html!(
                <li class="list-none flex items-center space-x-2">
                    <CheckBoxButton
                        class="basis-4 h-6"
                        checked={todo.completed}
                        on_click={(
                            props.checkable &&
                            (!todo.content.is_empty())
                        ).then(|| props.on_check_todo[i].clone())}
                    />
                    <TextInput
                        class="grow h-6 m-0 p-0"
                        value={todo.content.clone()}
                        on_change={(!todo.completed).then(|| props.on_edit_todo[i].clone())}
                    />
                </li>
            ))}
        </ul>
    )
}
