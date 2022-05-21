use yew::{Component, Context, Html, html, MouseEvent};
use super::navbar::{Navbar};

use crate::{utils::{Msg, getValue}, api::register, Route};
use yew_router::prelude::*;

pub struct Main;

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Navbar />
        }
    }
}