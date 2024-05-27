use yew::prelude::*;
use crate::models::{Card, Todo};
use super::atoms::{TextInput, CheckBoxButton};


#[derive(Properties, PartialEq)]
pub struct CardLayoutProps {
    pub title:    String,
    pub toolbox:  Html,
    pub contents: Html,

    #[prop_or("")]
    pub class:         &'static str,
    #[prop_or(None)]
    pub on_edit_title: Option<Callback<String>>,
    #[prop_or(None)]
    pub on_blur:       Option<Callback<()>>,
}

#[function_component]
pub fn CardLayout(props: &CardLayoutProps) -> Html {
    html!(
        <div class={props.class}>
            <div
                class="
                    bg-neutral-100
                    rounded-lg rounded-tr-none
                    border border-solid border-neutral-300
                    shadow-lg shadow-neutral-300
                    p-2 m-2
                "
                onblur={props.on_blur.as_ref().map(|h| h.reform(|_| ()))}
            >
                <header
                    class="h-8 space-x-2 flex items-center"
                >
                    <TextInput class="grow h-7 text-neutral-800 text-lg w-full h-full "
                        value={props.title.clone()}
                        on_input={props.on_edit_title.clone()}
                    />
                    <div class="basis-4 h-6">
                        {props.toolbox.clone()}
                    </div>
                </header>

                <hr class="border-neutral-400"/>

                {props.contents.clone()}
            </div>
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
        <ul class="m-0 p-0">{for props.todos.iter().enumerate().map(|(i, todo)| html!(
            <li class="list-none flex items-center space-x-2">
                    <CheckBoxButton
                        class="basis-4 h-6 cursor-pointer"
                        checked={todo.completed}
                        disabled={!props.checkable}
                        on_click={props.checkable.then(|| props.on_check_todo[i].clone()).unwrap_or_default()}
                    />
                    <TextInput
                        class="grow h-6 m-0 p-0"
                        value={todo.content.clone()}
                        disabled={todo.completed}
                        on_input={props.on_edit_todo[i].clone()}
                    />
            </li>
        ))}</ul>
    )
}
