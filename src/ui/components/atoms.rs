use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct DeleteButtonProps {
    #[prop_or_else(|| Callback::noop())]
    pub on_click: Callback<()>,

    #[prop_or("")]
    pub class:    &'static str,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component]
pub fn DeleteButton(props: &DeleteButtonProps) -> Html {
    html! (
        <div class={props.class}>
            <a
                onclick={props.on_click.reform(|_| ())}
                class={props.disabled.then_some("pointer-events-none")}
            >
                <img
                    src={if props.disabled {"assets/icons/delete_disabled.svg"} else {"assets/icons/delete.svg"}}
                />
            </a>
        </div>
    )
}


#[derive(Properties, PartialEq)]
pub struct CheckBoxButtonProps {
    pub on_click: Callback<()>,
    pub checked:  bool,

    #[prop_or("")]
    pub class:    &'static str,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component]
pub fn CheckBoxButton(props: &CheckBoxButtonProps) -> Html {
    html! (
        <div class={props.class}>
            <a
                onclick={props.on_click.reform(|_| ())}
                class={props.disabled.then_some("pointer-events-none")}
            >
                <img
                    src={if props.checked {"assets/icons/check_box.svg"} else {"assets/icons/check_box_outline_blank.svg"}}
                />
            </a>
        </div>
    )
}


#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub value:    String,
    pub on_input: Callback<String>,

    #[prop_or("")]
    pub class:    &'static str,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component]
pub fn TextInput(props: &TextInputProps) -> Html {
    let TextInputProps { class, value, on_input, disabled } = props;

    let on_input = on_input.reform(|e: Event| {
        use web_sys::{HtmlTextAreaElement, wasm_bindgen::JsCast};
        e.target().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value()
    });

    html!(
        <div class={*class}>
            <div class={if *disabled {"text-neutral-400"} else {"text-neutral-800"}}>
                <textarea
                    class="resize-none border-none w-full h-full outline-none bg-inherit"
                    autocomplete="off"
                    spellcheck="false"
                    value={value.clone()}
                    onchange={on_input}
                />
            </div>
        </div>
    )
}
