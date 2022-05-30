use gloo_net::Error;
use gloo_storage::{LocalStorage, Storage};
use yew::{Component, Context, Html, html, MouseEvent, function_component};
use crate::{types::{PrivateBoard, TeamBoard}, api::{get_private_boards, get_team_boards, delete_private, delete_team_board}, utils::map_token};
use yew_router::prelude::*;
use super::navbar::{Navbar};
use crate::Route;


pub struct TeamDetails {
    pub token: String
}

pub enum MsgBoard {
    Delete,
    Return
}

impl Component for TeamDetails {
    type Message = MsgBoard;
    type Properties = TeamBoard;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            token: map_token(LocalStorage::get("Token")).unwrap(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
        Self::Message::Delete => {
            let token = self.token.clone();
            let id = ctx.props().id;
            ctx.link().send_future(async move {
                delete_team_board(&token, id.unwrap()).await;
                Self::Message::Return
            });
            false
            }
        _ => {true}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="card" style="width: 18rem;">
                <div class="card-body">
                    <h5 class="card-title">{&ctx.props().name}</h5>
                    <h6 class="card-subtitle mb-2 text-muted">{"Team:"}{&ctx.props().team_name}</h6>
                    <a href={"board?board_type=team&&id=".to_owned() + ctx.props().id.unwrap().to_string().as_str()} class="btn btn-primary" role="button" aria-pressed="true">{"Open"}</a>
                    <button class="btn btn-danger" onclick={ctx.link().callback(|e: MouseEvent| {Self::Message::Delete})}>{"Delete"}</button>
                </div>
            </div>
        }
    }
}



pub struct PrivateDetails {
    pub token: String
}

impl Component for PrivateDetails {
    type Message = MsgBoard;
    type Properties = PrivateBoard;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            token: map_token(LocalStorage::get("Token")).unwrap(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
        Self::Message::Delete => {
            let token = self.token.clone();
            let id = ctx.props().id;
            ctx.link().send_future(async move {
                delete_private(&token, id.unwrap()).await;
                Self::Message::Return
            });
            false
            }
        _ => {true}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="card" style="width: 18rem;">
                <div class="card-body">
                    <h5 class="card-title">{&ctx.props().name}</h5>
                    <a href={"board?board_type=private&&id=".to_owned() + ctx.props().id.unwrap().to_string().as_str()} class="btn btn-primary" role="button" aria-pressed="true">{"Open"}</a>
                    <button class="btn btn-danger" onclick={ctx.link().callback(|e: MouseEvent| {Self::Message::Delete})}>{"Delete"}</button>
                </div>
            </div>
        }
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