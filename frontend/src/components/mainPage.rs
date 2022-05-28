use gloo_net::Error;
use gloo_storage::{LocalStorage, Storage};
use yew::{Component, Context, Html, html, MouseEvent, function_component};
use crate::{types::{PrivateBoard, TeamBoard}, api::{get_private_boards, get_team_boards}};
use yew_router::prelude::*;
use super::navbar::{Navbar};
use crate::Route;

#[function_component(TeamDetails)]
fn team_details(TeamBoard {team_name, id, name, owner}: &TeamBoard) -> Html {
    html! {
        <div class="card" style="width: 18rem;">
            <div class="card-body">
                <h5 class="card-title">{name}</h5>
                <h6 class="card-subtitle mb-2 text-muted">{"Team:"}{team_name}</h6>
                <a href="#" class="card-link">{"Open"}</a>
            </div>
        </div>
    }
}

#[function_component(PrivateDetails)]
fn private_details(PrivateBoard {name, owner, id}: &PrivateBoard) -> Html {
    html! {
        <div class="card" style="width: 18rem;">
            <div class="card-body">
                <h5 class="card-title">{name}</h5>
                <a href="#" class="card-link">{"Open"}</a>
            </div>
        </div>
    }
}

pub struct Main {
    private_boards: Option<Vec<PrivateBoard>>,
    team_boards: Option<Vec<TeamBoard>>,
    token: Option<String>
}

pub enum Msg{
    Private(Result<Vec<PrivateBoard>, Error>),
    Team(Result<Vec<TeamBoard>, Error>),
}


impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let token = LocalStorage::get("Token");
        match token {
            Ok(key) => Self {
                private_boards: None,
                team_boards: None,
                token: Some(key)
            },
            Err(_) => Self { private_boards: None, team_boards: None, token: None }
        }

    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Private(boards) => {
                if boards.is_err() {
                    return false;
                }
                self.private_boards = Some(boards.unwrap());
            }
            Self::Message::Team(boards) => {
                if boards.is_err() {
                    return false;
                }
                self.team_boards = Some(boards.unwrap());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.token.is_none() {
            return html! {<Redirect<Route> to={Route::Login}/>};
        }
        if self.private_boards.is_none() {
            let token = self.token.clone().unwrap();
            ctx.link().send_future(async move {
                let boards = get_private_boards(&token).await;
                Self::Message::Private(boards)
            });
            return html! {};
        }
        if self.team_boards.is_none() {
            let token = self.token.clone().unwrap();
            ctx.link().send_future(async move{
                let boards = get_team_boards(&token).await;
                Self::Message::Team(boards)
            });
            return html! {};
        }
        let privates = self.private_boards.clone();
        let privates = privates.unwrap().into_iter().map(|private| html! {
                        <PrivateDetails name={private.name.clone()} id={private.id.clone()} owner={private.owner.clone()} />
                    });
        
        let teams = self.team_boards.clone();
        let teams = teams.unwrap().into_iter().map(|team| html! {
                        <TeamDetails team_name={team.team_name.clone()} id={team.id.clone()} name={team.name.clone()} owner={team.owner.clone()}/>
                    });
        html! {
            <>
            <Navbar />
            <h1>{"Private boards"}</h1>
            {for privates}
            <h1>{"Team boards"}</h1>
            {for teams}
            </>
        }
    }
}