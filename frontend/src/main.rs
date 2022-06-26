use components::board::Board;
use components::login::LoginForm;
use components::main_page::Main;
use components::private_board_create::PrivateBoardCreate;
use components::register::RegisterForm;
use components::team_board_create::TeamBoardCreate;
use components::team_create::TeamCreate;
use components::timer::TimerList;
use yew::prelude::*;

mod api;
mod components;
mod types;
mod utils;

use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/index")]
    Main,
    #[at("private/create")]
    PrivateBoardCreate,
    #[at("team/create")]
    TeamCreate,
    #[at("team_board/create")]
    TeamBoardCreate,
    #[at("board")]
    Board,
    #[at("timers")]
    TimerList,
    #[not_found]
    #[at("/")]
    NotFound
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
        Route::Register => html! { <RegisterForm /> },
        Route::Main => html! {<Main />},
        Route::PrivateBoardCreate => html! {<PrivateBoardCreate />},
        Route::TeamCreate => html! {<TeamCreate />},
        Route::TeamBoardCreate => html!(<TeamBoardCreate />),
        Route::Board => html!(<Board />),
        Route::TimerList => html!(<TimerList />),
        Route::NotFound => html!(<Main />)
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
