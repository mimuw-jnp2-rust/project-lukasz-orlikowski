use super::navbar::Navbar;
use crate::api::create_private_board;
use crate::utils::{get_value, Msg};
use crate::Route;
use gloo_storage::{LocalStorage, Storage};
use yew::{html, Component, Context, Html, MouseEvent};
use yew_router::prelude::*;

pub struct PrivateBoardCreate {
    error: bool,
    success: bool,
    token: Option<String>,
}

impl Component for PrivateBoardCreate {
    type Message = Msg<bool>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let token = LocalStorage::get("Token");
        match token {
            Ok(key) => Self {
                error: false,
                success: false,
                token: Some(key),
            },
            Err(_) => Self {
                error: false,
                success: false,
                token: None,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let token = self.token.clone().unwrap();
        match msg {
            Self::Message::Submit => {
                ctx.link().send_future(async move {
                    let name = get_value("boardName");
                    let res = create_private_board(name.as_str(), token.as_str()).await;
                    Self::Message::Res(res)
                });
                false
            }
            Self::Message::Res(Ok(true)) => {
                self.error = false;
                self.success = true;
                true
            }
            _ => {
                self.error = true;
                self.success = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.success {
            return html! { <Redirect<Route> to={Route::Main}/> };
        }
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
