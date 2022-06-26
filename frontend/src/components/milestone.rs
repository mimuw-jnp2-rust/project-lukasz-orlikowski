use crate::api::create_milestone;
use crate::types::{BoardProp, MilestoneCreate};
use crate::utils::{get_value, reload};
use crate::Route;
use gloo_storage::{LocalStorage, Storage};
use yew::{html, Component, Context, Html, MouseEvent};
use yew_router::prelude::*;


pub struct MilestoneList {
    token: Option<String>,
}

pub enum Msg {
    Submit,
    Ok
}

impl Component for MilestoneList {
    type Message = Msg;
    type Properties = BoardProp;

    fn create(_ctx: &Context<Self>) -> Self {
        let token = LocalStorage::get("Token");
        match token {
            Ok(key) => Self {
                token: Some(key),
            },
            Err(_) => Self {
                token: None,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let token = self.token.clone().unwrap();
        match msg {
            Self::Message::Submit => {
                let milestone = MilestoneCreate {
                    id: None,
                    name: get_value("nameMilestone"),
                    board_id: ctx.props().id,
                    board_type: ctx.props().board_type.clone()
                };
                ctx.link().send_future(async move {
                    let _ = create_milestone(&token, milestone).await;
                    let _ = reload();
                    Self::Message::Ok
                });
                false
            }
            Self::Message::Ok => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.token {
            None => html! { <Redirect<Route> to={Route::Login}/> },
            _ => {
                    let milestones = ctx.props().milestones.clone();
                    let milestones = milestones.unwrap().into_iter().map(|milestone| html! {
                        <div class="card" style="width: 18rem;">
                            <div class="card-body">
                                <h5 class="card-title">{milestone.name}</h5>
                                <h6 class="card-subtitle mb-2 text-muted">{"Done:"}{milestone.done}</h6>
                                <h6 class="card-subtitle mb-2 text-muted">{"Total:"}{milestone.total}</h6>
                            </div>
                        </div>
                    });
                    html! {
                        <div>
                            <div class="col-xs-6" style="padding-left: 80px;">
                                <h1>{"Milestones"}</h1>
                                {for milestones}
                                <form>
                                    <div class="form-group">
                                        <label for="nameMilestone">{"Name"}</label>
                                        <input type="text" class="form-control" id="nameMilestone" aria-describedby="emailHelp" placeholder="Enter name"/>
                                    </div>
                                    <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Msg::Submit})}>{"Submit"}</button>
                                </form>
                            </div>
                        </div>
                    }
                }
        }
    }

}
