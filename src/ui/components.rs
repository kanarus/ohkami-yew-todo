use yew::prelude::*;


#[derive(Properties, PartialEq)]
struct DeleteButtonProps {
    on_click: Callback<()>,
    disabled: bool,
}

#[function_component]
pub fn DeleteButton(props: &DeleteButtonProps) -> Html {
    html! (
        <a
            onclick={props.on_click.reform(|_| ())}
            class={props.disabled.then_some("pointer-events-none")}
        >
            <img
                src={if props.disabled {"assets/icons/delete_disabled.svg"} else {"assets/icons/delete.svg"}}
            />
        </a>
    )
}


#[derive(Properties, PartialEq)]
struct CheckBoxButtonProps {
    on_click: Callback<()>,
    checked:  bool,
    disabled: bool,
}

#[function_component]
pub fn CheckBoxButton(props: &CheckBoxButtonProps) -> Html {
    html! (
        <a
            onclick={props.on_click.reform(|_| ())}
            class={props.disabled.then_some("pointer-events-none")}
        >
            <img
                src={if props.checked {"assets/icons/check_box.svg"} else {"assets/icons/check_box_outline_blank.svg"}}
            />
        </a>
    )
}


#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    #[prop_or_default]
    pub class:    &'static str,
    pub value:    String,
    pub on_input: Callback<String>,
}

#[function_component]
pub fn TextInput(props: &TextInputProps) -> Html {
    html!(
        <div class={props.class}>
            <textarea
                autocomplete="off"
                spellcheck="false"
                value={props.value.clone()}
                class="resize-none border-none w-full h-full outline-none bg-inherit"
            />
        </div>
    )
}
