use crate::api::login;
use crate::types::LoginResponse;
use crate::utils::{getValue, Msg};
use crate::Route;
use gloo_storage::{LocalStorage, Storage};
use yew::{html, Component, Context, Html, MouseEvent};
use yew_router::prelude::*;

pub struct LoginForm {
    error: bool,
    login: bool,
}

impl Component for LoginForm {
    type Message = Msg<LoginResponse>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: false,
            login: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Submit => {
                ctx.link().send_future(async {
                    let name = getValue("loginName");
                    let pass = getValue("loginPass");
                    let res = login(name.as_str(), pass.as_str()).await;
                    Self::Message::Res(res)
                });
                false
            }
            Self::Message::Res(Ok(res)) => {
                self.error = false;
                self.login = true;
                let _ = LocalStorage::set("Token", res.token);
                true
            }
            _ => {
                self.error = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.login {
            return html! {<Redirect<Route> to={Route::Main}/>};
        }
        html! {
            <div class="d-flex justify-content-md-center align-items-center vh-100">
                <div>
                <form>
                    <div class="form-group">
                        <label for="exampleInputEmail1">{"Username"}</label>
                        <input type="text" class="form-control" id="loginName" aria-describedby="emailHelp" placeholder="Enter username"/>
                    </div>
                    <div class="form-group">
                        <label for="exampleInputPassword1">{"Password"}</label>
                        <input type="password" class="form-control" id="loginPass" placeholder="Password" />
                    </div>
                    <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Msg::Submit})}>{"Submit"}</button>
                </form>
                if self.error {
                    <p style="color: red;">{"Error occured. Please try again."}</p>
                }
                    <center>
                        <Link<Route> to={Route::Register}>{ "Sign up" }</Link<Route>>
                    </center>
                </div>
            </div>
        }
    }
}
