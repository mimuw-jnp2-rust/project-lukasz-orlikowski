use gloo_storage::errors::StorageError;
use yew::{html, Component, Context, Html, MouseEvent};
use gloo_storage::{LocalStorage, Storage};
use crate::Route;
use crate::api::{login, create_private_board};
use crate::types::{LoginResponse};
use crate::utils::{getValue, Msg};
use yew_router::prelude::*;
use super::navbar::Navbar;


pub struct PrivateBoardCreate {
    error: bool,
    success: bool,
    token: Option<String>
}


impl Component for PrivateBoardCreate {
    type Message = Msg<bool>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let token = LocalStorage::get("Token");
        match token {
            Ok(key) => 
                Self {
                    error: false,
                    success: false,
                    token: Some(key)
                },
            Err(_) =>
                Self {
                    error: false,
                    success: false,
                    token: None
                }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let token = self.token.clone().unwrap();
        match msg {
            Self::Message::Submit => {
                ctx.link().send_future(async move {
                    let name = getValue("boardName");
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
                self.success = true;
                true
            }
        }
    }
    

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.token {
            None => html! { <Redirect<Route> to={Route::Login}/> },
            _ => 
                html! {
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
                }
        }
    }
}