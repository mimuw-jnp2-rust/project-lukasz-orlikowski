use yew::{Component, Context, Html, html, MouseEvent};

use crate::{utils::{Msg, getValue}, api::register, Route};
use yew_router::prelude::*;

pub struct RegisterForm {
    error: bool,
    success: bool
}


impl Component for RegisterForm {
    type Message = Msg<bool>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: false,
            success: false
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Submit => {
                ctx.link().send_future(async {
                    let name = getValue("registerName");
                    let pass = getValue("registerPass");
                    let res = register(name.as_str(), pass.as_str()).await;
                    Self::Message::Res(res)
                });
                false
            }
            Self::Message::Res(Ok(_)) => {
                self.error = false;
                self.success = true;
                true
            }
            _ => {
                self.error = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="d-flex justify-content-md-center align-items-center vh-100">
                <div>
                <form>
                    <div class="form-group">
                        <label for="exampleInputEmail1">{"username"}</label>
                        <input type="text" class="form-control" id="registerName" aria-describedby="usernameHelp" placeholder="Enter username"/>
                    </div>
                    <div class="form-group">
                        <label for="exampleInputPassword1">{"Password"}</label>
                        <input type="password" class="form-control" id="registerPass" placeholder="Password" />
                    </div>
                    <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Msg::Submit})}>{"Submit"}</button>
                </form>
                if self.error {
                    <p style="color: red;">{"Error occured. Please try again."}</p>
                }
                if self.success {
                    <p style="color: green;">{"Success. Please sign in."}</p>
                }
                    <center>
                        <Link<Route> to={Route::Login}>{ "Sign in" }</Link<Route>>
                    </center>
                </div>
            </div>
        }
    }
}