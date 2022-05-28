use gloo_net::Error;
use gloo_storage::{Storage, LocalStorage};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{Component, Context, Html, html, MouseEvent, function_component, Properties};

use crate::{utils::{getValue, map_token}, api::{register, get_lists, create_list}, Route, types::List};
use yew_router::prelude::*;
use super::navbar::Navbar;



#[wasm_bindgen(
    inline_js = "export function getParameter(input) { 
        var url_string = window.location.href;
        var url = new URL(url_string);
        var par = url.searchParams.get(input);
        return par;
     }"
)]

extern "C" {
    pub fn getParameter(input: &str) -> String;
}

#[function_component(ListDetails)]
fn list_details(Name {name}: &Name) -> Html {
    html! {
        <>
            <div class="col-xs-6" style="padding-left: 80px;">
                <h2>{name}</h2>
            </div>
            <div class="col-xs-6 vl"></div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct Name {
    pub name: String
}

pub struct Board {
    board_type: String,
    board_id: i32,
    lists: Option<Vec<List>>,
    token: Option<String>,
    error: bool
}

pub enum Msg {
    Submit,
    Res(Result<bool, Error>),
    Update(Result<Vec<List>, Error>),
}


impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let board_type = getParameter("board_type");
        let board_id = getParameter("id").parse::<i32>().unwrap();
        Self {
            board_type,
            board_id,
            lists: None,
            token: map_token(LocalStorage::get("Token")),
            error: false
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Submit => {
                let name = getValue("name");
                let board = self.board_id;
                let board_type = self.board_type.clone();
                let token = self.token.clone().unwrap();
                ctx.link().send_future(async move {
                    let res = create_list(&token, List{id: None, name, board, board_type}).await;
                    Self::Message::Res(res)
                });
                false
            }
            Self::Message::Update(Ok(lists)) => {
                self.lists = Some(lists);
                true
            }
            Self::Message::Res(Ok(_)) => {
                self.lists = None;
                true
            }
            _ => {
                self.error = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.token.is_none() {
            return html!{};
        }
        if self.lists.is_none() {
            let token = self.token.clone().unwrap();
            let board_id = self.board_id.clone();
            let board_type = self.board_type.clone();
            ctx.link().send_future(async move {
                let lists = get_lists(board_id, board_type, &token).await;
                Self::Message::Update(lists)
            });
            return html!{}
        }
        let lists = self.lists.clone();
        let lists = lists.unwrap().into_iter().map(|list| html! {
                        <ListDetails name={list.name}/>
                    });
        html! {
            <>
            <Navbar />
            <div class="row">
                {for lists}
                <div class="col-xs-6" style="padding-left: 80px;">
                    <form>
                        <div class="form-group">
                            <label for="name">{"name"}</label>
                            <input type="text" class="form-control" id="name" aria-describedby="usernameHelp" placeholder="Enter new list"/>
                        </div>
                        <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Msg::Submit})}>{"Submit"}</button>
                    </form>
                    if self.error {
                        <p style="color: red;">{"Error occured. Please try again."}</p>
                    }
                </div>
            </div>
            </>
        }
    }
}