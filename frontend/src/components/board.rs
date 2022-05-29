use gloo_net::Error;
use gloo_storage::{Storage, LocalStorage};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{Component, Context, Html, html, MouseEvent, function_component, Properties};

use crate::{utils::{getValue, map_token}, api::{register, get_lists, create_list, create_task, get_tasks, delete_task}, Route, types::{List, Task}};
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

#[wasm_bindgen(
    inline_js = "export function openModal() { 
        var modal = document.getElementById(\"myModal\");
        modal.style.display = \"block\";
        return true;
     }"
)]

extern "C" {
    pub fn openModal() -> bool;
}

#[wasm_bindgen(
    inline_js = "export function hideModal() { 
        var modal = document.getElementById(\"myModal\");
        modal.style.display = \"none\";
        return true;
     }"
)]

extern "C" {
    pub fn hideModal() -> bool;
}

#[function_component(ListOptions)]
fn list_options(List {id, name, board, board_type}: &List) -> Html {
    html! {
        <option value={id.unwrap().to_string()}>{name}</option>
    }
}



struct ListDetails {
    tasks: Option<Vec<Task>>,
    token: Option<String>
}

pub enum MsgList {
    Update(Result<Vec<Task>, Error>),
    Delete(Option<i32>),
    Return
}

impl Component for ListDetails {
    type Message = MsgList;
    type Properties = ListProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            tasks: None,
            token: map_token(LocalStorage::get("Token")),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Update(Ok(tasks)) => {self.tasks = Some(tasks);}
            Self::Message::Delete(id) => {
                let token = self.token.clone().unwrap();
                ctx.link().send_future(async move {
                    delete_task(&token, id.unwrap()).await;
                    Self::Message::Return
                });
            }
            _ => {}
        }
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.tasks.is_none() {
            let token = self.token.clone().unwrap();
            let id = ctx.props().id;
            ctx.link().send_future(async move {
                let tasks = get_tasks(&token, id).await;
                Self::Message::Update(tasks)
            });
            return html!{};
        }
        let tasks = self.tasks.clone();
        let tasks = tasks.unwrap().into_iter().map(|task| html! {
            <div class="card" style="width: 18rem;">
                <div class="card-body">
                    <h5 class="card-title">{task.name}</h5>
                    <h6 class="card-subtitle mb-2 text-muted">{"Note:"}{task.note.unwrap()}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Place:"}{task.place.unwrap()}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Assigned:"}{task.members.unwrap()}</h6>
                    <button class="btn btn-danger" onclick={ctx.link().callback(move |e: MouseEvent| {Self::Message::Delete(task.id)})}>{"Delete"}</button>
                </div>
            </div>
        });
        html! {
            <>
                <div class="col-xs-6" style="padding-left: 80px;">
                    <h2>{&ctx.props().name}</h2>
                    {for tasks}
                </div>
                <div class="col-xs-6 vl"></div>
            </>
        }
    }
}

#[derive(Properties, PartialEq)]
struct ListProp {
    pub name: String,
    pub id: i32,
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
    AddTask
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
            Self::Message::AddTask => {
                let name = getValue("nameTask");
                let note = getValue("note");
                let place = getValue("place");
                let members = getValue("members");
                let list = getValue("list").parse::<i32>().unwrap();
                let token = self.token.clone().unwrap();
                ctx.link().send_future(async move {
                    let res = create_task(&token, Task{id: None, name, note: Some(note), place: Some(place), members: Some(members), list}).await;
                    Self::Message::Res(res) 
                });
                false
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
                        <ListDetails name={list.name} id ={list.id.unwrap()}/>
                    });
        let lists_options = self.lists.clone();
        let lists_options = lists_options.unwrap().into_iter().map(|list| html! {
                                <ListOptions name={list.name.clone()} board={list.board.clone()} board_type={list.board_type.clone()} id={list.id.clone()} />
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
                <div class="col-xs-6" style="padding-left: 80px;">
                    <button class="btn btn-primary" id="myBtn" onclick={|e: MouseEvent| {openModal();}} >{"Add task"}</button>

                    <div id="myModal" class="modal">

                    <div class="modal-content">
                        <span class="close btn btn-danger" onclick={|e: MouseEvent| {hideModal();}}>{"Hide"}</span>
                        <form>
                        <div class="form-group">
                            <label for="name">{"name"}</label>
                            <input type="text" class="form-control" id="nameTask" aria-describedby="usernameHelp" placeholder="Enter new task name"/>
                        </div>
                        <div class="form-group">
                            <label for="note">{"note"}</label>
                            <input type="text" class="form-control" id="note" aria-describedby="usernameHelp" placeholder="Enter note"/>
                        </div>
                        <div class="form-group">
                            <label for="place">{"place"}</label>
                            <input type="text" class="form-control" id="place" aria-describedby="usernameHelp" placeholder="Enter place"/>
                        </div>
                        <div class="form-group">
                            <label for="members">{"Assigned people"}</label>
                            <input type="text" class="form-control" id="members" aria-describedby="usernameHelp" placeholder="Enter assigned people"/>
                            <small id="emailHelp" class="form-text text-muted">{"Assigned people should be seperated by ;"}</small>
                        </div>
                        <label for="team">{"Choose list:"}</label>
                            <select id="list" name="team">
                                {for lists_options}
                            </select>
                        <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Msg::AddTask})}>{"Submit"}</button>
                    </form>
                    </div>

                    </div>
                </div>
            </div>
            </>
        }
    }
}