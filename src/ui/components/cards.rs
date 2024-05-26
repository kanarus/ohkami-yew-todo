use yew::prelude::*;
use super::atoms::{TextInput, DeleteButton, CheckBoxButton};
use crate::models::{Card, CreateCardRequest};


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

    use_effect_with(input.clone(), {
        let handler = props.handler.clone();
        move |input| handler.on_initial_input.emit(input.clone())
    });

    html!(
        <div class="bg-neutral-100 rounded-lg rounded-tr-none border border-solid border-neutral-300 shadow-lg shadow-neutral-300 p-2 m-2">
            <header class="h-8 space-x-2 flex items-center">
                <TextInput
                    class="grow h-7 text-neutral-800 text-lg"
                    value={input.title.clone()}
                    on_input={Callback::from({
                        let input = input.clone();
                        move |value| input.set({
                            let mut new_input = (&*input).clone();
                            new_input.title = value;
                            new_input
                        })
                    })}
                />
                <DeleteButton
                    class="basis-4 h-6"
                    disabled={true}
                    on_click={Callback::noop()}
                />
            </header>

            <hr class="border-neutral-400"/>

            <ul>{for input.todos.iter().enumerate().map(|(i, todo)| html!(
                <li class="list-none flex items-center space-x-2">
                    <div class={"text-neutral-800"}>
                        <CheckBoxButton
                            class="basis-4 h-6"
                            checked={false}
                            disabled={true}
                            on_click={Callback::noop()}
                        />
                        <TextInput
                            class="grow h-6 m-0 p-0"
                            value={todo.clone()}
                            on_input={Callback::from({
                                let input = input.clone();
                                move |value| input.set({
                                    let mut new_input = (&*input).clone();
                                    new_input.todos[i] = value;
                                    new_input
                                })
                            })}
                        />
                    </div>
                </li>
            ))}</ul>
        </div>
    )
}
