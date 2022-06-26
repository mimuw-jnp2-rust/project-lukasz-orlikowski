use super::navbar::Navbar;
use crate::api::{get_timers, update_timer, delete_timer, create_timer};
use crate::types::Timer;
use crate::utils::{get_value, err};
use crate::Route;
use gloo_net::Error;
use gloo_storage::{LocalStorage, Storage};
use yew::{html, Component, Context, Html, MouseEvent};
use yew_router::prelude::*;
use gloo_timers::callback::{Interval};


pub struct TimerList {
    token: Option<String>,
    timers: Option<Vec<Timer>>,
    _clock_handle: Interval
}

pub enum Msg {
    Fetch,
    Update,
    Submit,
    Res(Result<Vec<Timer>, Error>),
    UpdateTimer(i32),
    Delete(i32),
    Ok
}

impl Component for TimerList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let token = LocalStorage::get("Token");
        let _clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1000, move || link.send_message(Self::Message::Update))
        };
        match token {
            Ok(key) => Self {
                token: Some(key),
                timers: None,
                _clock_handle
            },
            Err(_) => Self {
                token: None,
                timers: None,
                _clock_handle
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let token = self.token.clone().unwrap();
        match msg {
            Self::Message::Fetch => {
                ctx.link().send_future(async move {
                    let res = get_timers(token.as_str()).await;
                    Self::Message::Res(res)
                });
                false
            }
            Self::Message::Res(Ok(res)) => {
                self.timers = Some(res);
                true
            }
            Self::Message::Update => {
                true
            }
            Self::Message::UpdateTimer(x) => {
                ctx.link().send_future(async move {
                    let _ = update_timer(token.as_str(), x).await;
                    Self::Message::Ok
                });
               
                false
            }
            Self::Message::Ok => {
                self.timers = None;
                true
            }
            Self::Message::Delete(x) => {
                ctx.link().send_future(async move {
                    let _ = delete_timer(token.as_str(), x).await;
                    Self::Message::Ok
                });
                false
            }
            Self::Message::Submit => {
                let name = get_value("name");
                ctx.link().send_future(async move {
                    let _ = create_timer(token.as_str(), name.as_str()).await;
                    Self::Message::Ok
                });
                false
            }
            _ => {
                let _ = err("Error occured");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.token {
            None => html! { <Redirect<Route> to={Route::Login}/> },
            _ => if self.timers.is_none() {
                    ctx.link().send_message(Self::Message::Fetch);
                    html! {}
                }
                else {
                    let timers = self.timers.clone();
                    let timers = timers.unwrap().into_iter().map(|timer| html! {
                        <div class="card" style="width: 18rem;">
                            <div class="card-body">
                                <h5 class="card-title">{timer.name.clone()}</h5>
                                <h6 class="card-subtitle mb-2 text-muted">{"Time:"}{timer.get_time()}</h6>
                                <button class="btn btn-danger" onclick={ctx.link().callback(move |_: MouseEvent| {Self::Message::Delete(timer.id)})}>{"Delete"}</button>
                                if timer.status == "stopped" {
                                    <button class="btn btn-success" onclick={ctx.link().callback(move |_: MouseEvent| {Self::Message::UpdateTimer(timer.id)})}>{"Start"}</button>
                                }
                                else {
                                    <button class="btn btn-danger" onclick={ctx.link().callback(move |_: MouseEvent| {Self::Message::UpdateTimer(timer.id)})}>{"Stop"}</button>
                                }
                            </div>
                        </div>
                    });
            
                    html! {
                        <div>
                            <Navbar />
                            <div class="col-xs-6" style="padding-left: 80px;">
                                {for timers}
                            </div>
                            <form>
                                <div class="form-group">
                                    <label for="exampleInputEmail1">{"Name"}</label>
                                    <input type="text" class="form-control" id="name" aria-describedby="emailHelp" placeholder="Enter name"/>
                                </div>
                                <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Msg::Submit})}>{"Submit"}</button>
                            </form>
                        </div>
                    }
                }
        }
    }

}
