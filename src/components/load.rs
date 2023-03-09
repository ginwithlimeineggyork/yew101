use yew::prelude::*;

#[function_component(Load)]
pub fn load() -> Html {
    html! {
    <div class="spinner-border" role="status">
        <span class="visually-hidden">{"Loading..."}</span>
    </div>
    }
}
