use yew::prelude::*;
use components::login::{LoginForm};
use components::register::{RegisterForm};

mod components;
mod api;
mod types;
mod utils;

use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
}

#[function_component(App)]
fn app() -> Html {
    html! {
    <BrowserRouter>
        <Switch<Route> render={Switch::render(switch)} />
    </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! { <LoginForm /> },
        Route::Register => html! { <RegisterForm /> }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}