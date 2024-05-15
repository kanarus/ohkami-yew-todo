mod ui;
mod models;

use yew::prelude::*;


#[function_component]
fn App() -> Html {
    let count = use_state(|| 0);

    let onclick = Callback::from({
        let count = count.clone();
        move |_| count.set(*count + 1)
    });

    html! {
        <>
            <h1 class="w-full text-center">{"TODO"}</h1>
            <div class="w-full text-center">
                <button class="text-orange-500" {onclick}>{*count}</button>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
