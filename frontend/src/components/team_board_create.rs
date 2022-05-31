use super::navbar::Navbar;
use crate::api::{create_team_board, get_user_teams};
use crate::types::Team;
use crate::utils::getValue;
use crate::Route;
use gloo_net::Error;
use gloo_storage::{LocalStorage, Storage};
use yew::{function_component, html, Component, Context, Html, MouseEvent};
use yew_router::prelude::*;

pub struct TeamBoardCreate {
    error: bool,
    success: bool,
    token: Option<String>,
    teams: Option<Vec<Team>>,
}

pub enum Msg {
    Submit,
    Res(Result<bool, Error>),
    Update(Result<Vec<Team>, Error>),
    Fetch,
}

#[function_component(TeamDetails)]
fn team_details(Team { id, name, owner }: &Team) -> Html {
    let _ = owner;
    html! {
        <option value={id.to_string()}>{name}</option>
    }
}

impl Component for TeamBoardCreate {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let token = LocalStorage::get("Token");
        match token {
            Ok(key) => Self {
                error: false,
                success: false,
                token: Some(key),
                teams: None,
            },
            Err(_) => Self {
                error: false,
                success: false,
                token: None,
                teams: None,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let token = self.token.clone().unwrap();
        match msg {
            Self::Message::Submit => {
                ctx.link().send_future(async move {
                    let name = getValue("boardName");
                    let team = getValue("teamName");
                    let res = create_team_board(
                        name.as_str(),
                        team.parse::<i32>().unwrap(),
                        token.as_str(),
                    )
                    .await;
                    Self::Message::Res(res)
                });
                false
            }
            Self::Message::Res(Ok(true)) => {
                self.error = false;
                self.success = true;
                true
            }
            Self::Message::Fetch => {
                ctx.link().send_future(async move {
                    let teams = get_user_teams(token.as_str()).await;
                    Self::Message::Update(teams)
                });
                false
            }
            Self::Message::Update(Ok(teams)) => {
                self.teams = Some(teams);
                true
            }
            _ => {
                self.error = true;
                self.success = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.teams.is_none() {
            ctx.link().send_message(Self::Message::Fetch);
            html! {}
        } else {
            let teams_copy = self.teams.clone();
            let teams = teams_copy.unwrap().into_iter().map(|team| {
                html! {
                    <TeamDetails id={team.id} owner={team.owner} name={team.name.clone()} />
                }
            });
            match self.token {
                None => html! { <Redirect<Route> to={Route::Login}/> },
                _ => html! {
                    <div>
                        <Navbar />
                        <div class="d-flex justify-content-md-center align-items-center vh-100">
                            <div>
                            <form>
                                <div class="form-group">
                                    <label for="boardName">{"Name"}</label>
                                    <input type="email" class="form-control" id="boardName" aria-describedby="nameHelp" placeholder="Enter name"/>
                                </div>
                                <label for="team">{"Choose a team:"}</label>
                                    <select id="teamName" name="team">
                                        {for teams}
                                    </select>
                                <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Msg::Submit})}>{"Submit"}</button>
                            </form>
                            if self.error {
                                <p style="color: red;">{"Error occured. Please try again."}</p>
                            }
                            if self.success {
                                <p style="color: green;">{"Board created successfully."}</p>
                            }
                            </div>
                        </div>
                    </div>
                },
            }
        }
    }
}
