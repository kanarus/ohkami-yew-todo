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
pub fn CheckBoxButton(props: &CheckBoxButtonProps) -> Html {
    html! (
        <Button on_click={props.on_click.clone()} class={props.class}>
            <img src={
                if props.checked {
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
    pub on_input: Option<Callback<String>>,
}

#[function_component]
pub fn TextInput(TextInputProps {
    value,
    class,
    is_title,
    on_input,
}: &TextInputProps) -> Html {
    let on_edit = on_input.clone().unwrap_or_else(Callback::noop).reform(|e: InputEvent| {
        use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
        e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()
    });

    html!(
        <div class={*class}>
            <input
                class={match (*is_title, on_input.is_none()) {
                    (true,  _    ) => "text-lg   + text-neutral-800 + resize-none border-none w-full h-full outline-none bg-inherit",
                    (false, true ) => "text-base + text-neutral-400 + resize-none border-none w-full h-full outline-none bg-inherit",
                    (false, false) => "text-base + text-neutral-800 + resize-none border-none w-full h-full outline-none bg-inherit",
                }}
                autocomplete="off"
                spellcheck="false"
                disabled={on_input.is_none()}
                value={value.clone()}
                oninput={on_edit}
            />
        </div>
    )
}
