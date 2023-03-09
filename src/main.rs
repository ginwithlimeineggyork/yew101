use yew::prelude::*;

mod components;
mod models;

use gloo_net::{
    http::Request, 
    Error
};
use models::user::Users;

#[allow(unused_imports)]
use components::{
    card::Card, 
    nav::Nav, 
    load::Load, 
    msg::Msg, 
    input::Input
};

#[function_component(App)]
fn app() -> Html {
    let users: UseStateHandle<Option<Users>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);

    {
        //create copies of states
        let users = users.clone();
        let error = error.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_users = Request::get(
                        "https://dummyjson.com/users").send().await;
                    match fetched_users {
                        Ok(response) => {
                            let json = response.json::<Users>().await;
                            match json {
                                Ok(json_resp) => {
                                    users.set(Some(json_resp));
                                }
                                Err(e) => error.set(Some(e)),
                            }
                        }
                        Err(e) => error.set(Some(e)),
                    }
                });
                || ()
            },
            (),
        );
    }

    let user_list_logic = match users.as_ref() {
        Some(users) => users
            .users
            .iter()
            .map(|user| {
                html! {
                  <Card user={user.clone() }/>
                }
            })
            .collect(),
        None => match error.as_ref() {
            Some(_) => {
                html! {
                    <Msg 
                    text={"Error getting list of users"} 
                    css_class={"text-danger"}/>
                }
            }
            None => {
                html! {
                  <Load />
                }
            }
        },
    };

    log::debug!("{:?}",user_list_logic);

    html! {
      <>
        <Nav />
        <Input />  
        {user_list_logic}
      </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("App is starting");
    yew::start_app::<App>();
}
