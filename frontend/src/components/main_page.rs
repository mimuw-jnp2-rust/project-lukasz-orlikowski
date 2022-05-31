use super::navbar::Navbar;
use crate::Route;
use crate::{
    api::{delete_private, delete_team_board, get_private_boards, get_team_boards, update_board},
    types::{PrivateBoard, TeamBoard},
    utils::{getValue, hideModal, map_token, openModal, reload, setValue},
};
use gloo_net::Error;
use gloo_storage::{LocalStorage, Storage};
use yew::{html, Component, Context, Html, MouseEvent};
use yew_router::prelude::*;

pub struct TeamDetails {
    pub token: String,
}

pub enum MsgBoard {
    Delete,
    Return,
    Update,
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
                    let _ = delete_team_board(&token, id.unwrap()).await;
                    Self::Message::Return
                });
                false
            }
            Self::Message::Update => {
                let name = ctx.props().name.as_str();
                let id = ctx.props().id.unwrap();
                setValue("nameBoard", name);
                setValue("idBoard", id.to_string().as_str());
                setValue("typeBoard", "team");
                false
            }
            _ => {
                let _ = reload();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="card" style="width: 18rem;">
                <div class="card-body">
                    <h5 class="card-title">{&ctx.props().name}</h5>
                    <h6 class="card-subtitle mb-2 text-muted">{"Team:"}{&ctx.props().team_name}</h6>
                    <a href={"board?board_type=team&&id=".to_owned() + ctx.props().id.unwrap().to_string().as_str()} class="btn btn-primary" role="button" aria-pressed="true">{"Open"}</a>
                    <button class="btn btn-danger" onclick={ctx.link().callback(|_: MouseEvent| {Self::Message::Delete})}>{"Delete"}</button>
                    <button class="btn btn-primary" onclick={ctx.link().callback(|_: MouseEvent| {openModal("myModal"); Self::Message::Update})}>{"Update"}</button>
            </div>
            </div>
        }
    }
}

pub struct PrivateDetails {
    pub token: String,
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
                    let _ = delete_private(&token, id.unwrap()).await;
                    Self::Message::Return
                });
                false
            }
            Self::Message::Update => {
                let name = ctx.props().name.as_str();
                let id = ctx.props().id.unwrap();
                setValue("nameBoard", name);
                setValue("idBoard", id.to_string().as_str());
                setValue("typeBoard", "private");
                false
            }
            _ => {
                let _ = reload();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="card" style="width: 18rem;">
                <div class="card-body">
                    <h5 class="card-title">{&ctx.props().name}</h5>
                    <a href={"board?board_type=private&&id=".to_owned() + ctx.props().id.unwrap().to_string().as_str()} class="btn btn-primary" role="button" aria-pressed="true">{"Open"}</a>
                    <button class="btn btn-danger" onclick={ctx.link().callback(|_: MouseEvent| {Self::Message::Delete})}>{"Delete"}</button>
                    <button class="btn btn-primary" onclick={ctx.link().callback(|_: MouseEvent| {openModal("myModal"); Self::Message::Update})}>{"Update"}</button>
                </div>
            </div>
        }
    }
}

pub struct Main {
    private_boards: Option<Vec<PrivateBoard>>,
    team_boards: Option<Vec<TeamBoard>>,
    token: Option<String>,
}

pub enum Msg {
    Private(Result<Vec<PrivateBoard>, Error>),
    Team(Result<Vec<TeamBoard>, Error>),
    UpdateBoard,
    Return,
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
                token: Some(key),
            },
            Err(_) => Self {
                private_boards: None,
                team_boards: None,
                token: None,
            },
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
            Self::Message::UpdateBoard => {
                let id = getValue("idBoard").parse::<i32>().unwrap();
                let name = getValue("nameBoard");
                let token = self.token.clone().unwrap();
                let board_type = getValue("typeBoard");
                ctx.link().send_future(async move {
                    let _ = update_board(&token, id, name, &board_type).await;
                    Self::Message::Return
                });
            }
            Self::Message::Return => {
                self.private_boards = None;
                self.team_boards = None;
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
            ctx.link().send_future(async move {
                let boards = get_team_boards(&token).await;
                Self::Message::Team(boards)
            });
            return html! {};
        }
        let privates = self.private_boards.clone();
        let privates = privates.unwrap().into_iter().map(|private| {
            html! {
                <PrivateDetails name={private.name.clone()} id={private.id} owner={private.owner} />
            }
        });

        let teams = self.team_boards.clone();
        let teams = teams.unwrap().into_iter().map(|team| html! {
                        <TeamDetails team_name={team.team_name} id={team.id} name={team.name.clone()} owner={team.owner}/>
                    });
        html! {
            <>
            <Navbar />
            <h1>{"Private boards"}</h1>
            {for privates}
            <h1>{"Team boards"}</h1>
            {for teams}
            <div id="myModal" class="modal">

            <div class="modal-content">
                <span class="close btn btn-danger" onclick={|_: MouseEvent| {hideModal("myModal");}}>{"Hide"}</span>
                <form>
                <div class="form-group">
                    <label for="name">{"name"}</label>
                    <input type="text" class="form-control" id="nameBoard" aria-describedby="usernameHelp" placeholder="Enter new board name"/>
                </div>
                <div class="form-group">
                    <input type="hidden" class="form-control" id="idBoard" aria-describedby="usernameHelp" placeholder="Enter place"/>
                </div>
                <div class="form-group">
                    <input type="hidden" class="form-control" id="typeBoard" aria-describedby="usernameHelp" placeholder="Enter assigned people"/>
                </div>
                <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Msg::UpdateBoard})}>{"Submit"}</button>
            </form>
            </div>

            </div>
            </>
        }
    }
}
