use yew::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
    <nav class="navbar bg-black">
        <div class="container-fluid">
            <a class="navbar-brand text-white" href="/">{"User List"}</a>
        </div>
    </nav>
    }
}
