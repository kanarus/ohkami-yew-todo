#[macro_export]
macro_rules! render {
    ( $( $yew_html_flagment:tt )* ) => {
        {
            #[::yew::prelude::function_component]
            #[allow(non_camel_case_types)]
            fn __yew_component__() -> ::yew::prelude::Html {
                ::yew::prelude::html! { $( $yew_html_flagment )* }
            }
            crate::render::HTML::new::<__yew_component__>().await
        }
    };
}

#[allow(unused)]
pub struct HTML(String);
const _: () = {
    #[allow(unused)]
    impl HTML {
        #[doc(hidden)]
        pub async fn new<Component>() -> Self
        where
        Component: yew::BaseComponent,
        Component::Properties: Default,
        {
            Self(yew::ServerRenderer::<Component>::new().render().await)
        }
    }

    use ohkami::prelude::*;
    impl IntoResponse for HTML {
        fn into_response(self) -> Response {
            Response::OK().with_html(self.0)
        }
    }
};
