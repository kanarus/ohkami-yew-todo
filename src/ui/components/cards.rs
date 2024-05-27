use yew::prelude::*;
use super::atoms::DeleteButton;
use super::layouts::{CardLayout, TodoLayout};
use crate::models::{Card, CreateCardRequest, Todo};



#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    pub bind:    Card,
    pub handler: TodoCardHandler,

    #[prop_or("")]
    pub class: &'static str,
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
            class={props.class}
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

    #[prop_or("")]
    pub class: &'static str,
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
        <CardLayout
            class={props.class}
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
                <DeleteButton
                    disabled={true}
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
