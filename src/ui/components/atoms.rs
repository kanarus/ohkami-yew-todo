use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or(None)]
    children: Option<Html>,

    #[prop_or(None)]
    pub on_click: Option<Callback<()>>,
    #[prop_or("")]
    pub class:    &'static str,
}

#[function_component]
fn Button(props: &ButtonProps) -> Html {
    html! (
        <div class={props.class}>
            <div class={props.on_click.is_some().then_some("cursor-pointer")}>
                <a
                    tabindex="0"
                    class={props.on_click.is_none().then_some("pointer-events-none")}
                    onclick={props.on_click.as_ref().map(|h| h.reform(|_| ()))}
                >
                    {props.children.clone()}
                </a>
            </div>
        </div>
    )
}


#[function_component]
pub fn DeleteButton(props: &ButtonProps) -> Html {
    html!(
        <Button on_click={props.on_click.clone()} class={props.class}>
            <img src={
                if props.on_click.is_none() {
                    "assets/icons/delete_disabled.svg"
                } else {
                    "assets/icons/delete.svg"
                }
            }/>
        </Button>
    )
}

#[function_component]
pub fn UploadButton(props: &ButtonProps) -> Html {
    html!(
        <Button on_click={props.on_click.clone()} class={props.class}>
            <img src={
                if props.on_click.is_none() {
                    "assets/icons/upload_disabled.svg"
                } else {
                    "assets/icons/upload.svg"
                }
            }/>
        </Button>
    )
}

#[derive(Properties, PartialEq)]
pub struct CheckBoxButtonProps {
    pub checked:  bool,
    
    #[prop_or("")]
    pub class:    &'static str,
    #[prop_or(None)]
    pub on_click: Option<Callback<()>>,
}

#[function_component]
pub fn CheckBoxButton(CheckBoxButtonProps {
    checked,
    class,
    on_click,
}: &CheckBoxButtonProps) -> Html {
    html! (
        <Button {on_click} class={class}>
            <img src={
                if *checked {
                    "assets/icons/check_box.svg"
                } else {
                    "assets/icons/check_box_outline_blank.svg"
                }
            }/>
        </Button>
    )
}


#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub value:    String,

    #[prop_or(false)]
    pub is_title: bool,
    #[prop_or("")]
    pub class: &'static str,
    #[prop_or(None)]
    pub on_change: Option<Callback<String>>,
    #[prop_or(None)]
    pub on_input:  Option<Callback<String>>,
}

#[function_component]
pub fn TextInput(TextInputProps {
    value,
    class,
    is_title,
    on_change,
    on_input,
}: &TextInputProps) -> Html {
    use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};

    let disabled = on_change.is_none() && on_input.is_none();

    html!(
        <div class={*class}>
            <input
                class={match (*is_title, disabled) {
                    (true,  _    ) => "text-lg   + text-neutral-800 + resize-none border-none w-full h-full outline-none bg-inherit",
                    (false, true ) => "text-base + text-neutral-400 + resize-none border-none w-full h-full outline-none bg-inherit",
                    (false, false) => "text-base + text-neutral-800 + resize-none border-none w-full h-full outline-none bg-inherit",
                }}
                autocomplete="off"
                spellcheck="false"
                disabled={disabled}
                value={value.clone()}
                onchange={on_change.as_ref().map(|h| h.reform(|e: Event| {
                    e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()
                }))}
                oninput={on_input.as_ref().map(|h| h.reform(|e: InputEvent| {
                    e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()
                }))}
            />
        </div>
    )
}
