use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageProp {
    pub text: String,
    pub css_class: String,
}

#[function_component(Msg)]
pub fn msg(MessageProp { text, css_class }: &MessageProp) -> Html {
    html! {
        <p class={css_class.clone()}>
            {text.clone()}
        </p>
    }
}
