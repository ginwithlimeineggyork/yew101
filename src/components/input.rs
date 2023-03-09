use yew::prelude::*;

#[function_component(Input)]
pub fn input() -> Html {
    html! {
    <div class="m-3 p-4 d-flex align-items-center justify-content-center">
            <div class="input-group mb-3">
                <input 
                    type="text" 
                    id="filter" 
                    class="form-control"
                    aria-label="user name" 
                    aria-describedby="button-addon2"
                    placeholder="filter for search" 
                />
                <input 
                    type="submit" 
                    class="btn btn-outline-secondary"
                    id="button-addon2"
                />
          </div>
    </div>
    }
}
