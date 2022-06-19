use gloo_net::Error;
use gloo_storage::{LocalStorage, Storage};
use yew::{function_component, html, Component, Context, Html, MouseEvent, Properties};
use super::milestone::MilestoneList;

use super::navbar::Navbar;
use crate::{
    api::{
        create_list, create_task, delete_list, delete_task, get_lists, get_task, get_tasks,
        update_task, get_logs, get_milestones,
    },
    types::{List, Task, TaskFilter, Log, IdProp, Milestone},
    utils::{getParameter, getValue, hideModal, map_token, openModal, reload, setValue, map_result, set_checked, is_checked},
};

#[function_component(ListOptions)]
fn list_options(
    List {
        id,
        name,
        board,
        board_type,
    }: &List,
) -> Html {
    let _ = board;
    let _ = board_type; // For clippy
    html! {
        <option value={id.unwrap().to_string()}>{name}</option>
    }
}

#[derive(PartialEq, Properties)]
pub struct SubTasksProps {
    pub subtasks: String,
}

#[function_component(SubTasks)]
fn sub_tasks(SubTasksProps { subtasks }: &SubTasksProps) -> Html {
    let sub_tasks = subtasks.split(';').into_iter().map(|subtask| {
        html! {
            <h6 class="card-subtitle mb-2 text-muted">{"Subtask:"}{subtask}</h6>
        }
    });
    html! {
        <>
        <h5 class="card-title">{"Subtasks"}</h5>
        {for sub_tasks}
        </>
    }
}

struct Logs {
    token: Option<String>,
    logs: Option<Vec<Log>>
}

pub enum MsgLogs {
    Update(Result<Vec<Log>, Error>),
}

impl Component for Logs {
    type Message = MsgLogs;
    type Properties = IdProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            logs: None,
            token: map_token(LocalStorage::get("Token")),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            if let Self::Message::Update(Ok(logs)) = msg {
                self.logs = Some(logs);
            }
            true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.logs.is_none() {
            let token = self.token.clone().unwrap();
            let id = ctx.props().id;
            ctx.link().send_future(async move {
                let logs = get_logs(id, &token).await;
                Self::Message::Update(logs)
            });
            return html! {};
        }

        let logs = self.logs.clone();
        let logs = logs.unwrap().into_iter().map(|log| html! {
            <div class="card" style="width: 18rem;">
                <div class="card-body">
                    <h5 class="card-title">{"Name: "}{log.name}{" Action: "}{log.action}{" When:"}{log.timestamp}</h5>
                    <h6 class="card-subtitle mb-2 text-muted">{"Note:"}{log.note.unwrap()}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Place:"}{log.place.unwrap()}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Assigned:"}{log.members.unwrap()}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Deadline:"}{log.deadline}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Points:"}{log.points}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Tags:"}{log.tags}</h6>
                    <SubTasks subtasks={log.subtasks.clone()}/>
                </div>
            </div>
        });

        let id_prop = ctx.clone().props().id;
        let id = format!("logs{}", id_prop);
        html! {
            <div id={id.clone()} class="modal">

                <div class="modal-content">
                    <span class="close btn btn-danger" onclick={move |_: MouseEvent| {hideModal(id.as_str());}}>{"Hide"}</span>
                    {for logs}
                </div>
            </div>
        }
    }
}

struct ListDetails {
    tasks: Option<Vec<Task>>,
    token: Option<String>,
    id: Option<i32>,
}

pub enum MsgList {
    Update(Result<Vec<Task>, Error>),
    Delete(Option<i32>),
    Return,
    UpdateTask(Option<i32>),
    DeleteList,
    Pass
}

impl Component for ListDetails {
    type Message = MsgList;
    type Properties = ListProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            tasks: None,
            token: map_token(LocalStorage::get("Token")),
            id: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Update(Ok(tasks)) => {
                self.tasks = Some(tasks);
                true
            }
            Self::Message::Delete(id) => {
                let token = self.token.clone().unwrap();
                ctx.link().send_future(async move {
                    let _ = delete_task(&token, id.unwrap()).await;
                    let _ = reload();
                    Self::Message::Return
                });
                false
            }
            Self::Message::DeleteList => {
                let id = ctx.props().id;
                let token = self.token.clone().unwrap();
                ctx.link().send_future(async move {
                    let _ = delete_list(&token, id).await;
                    let _ = reload();
                    Self::Message::Return
                });
                false
            }
            Self::Message::UpdateTask(id) => {
                self.id = id;
                let token = self.token.clone().unwrap();
                ctx.link().send_future(async move {
                    let task = get_task(&token, id.unwrap()).await;
                    if task.is_err() {
                        return Self::Message::Return;
                    }
                    let task = task.unwrap();

                    setValue("idUpdate", id.unwrap().to_string().as_str());
                    setValue("nameUpdateTask", task.name.as_str());
                    setValue("pointsUpdate", task.points.to_string().as_str());
                    setValue("tagsUpdate", task.tags.as_str());
                    if task.done == 1 {
                        set_checked("doneUpdate");
                    }

                    if task.note.is_some() {
                        setValue("noteUpdate", task.note.unwrap().as_str());
                    } else {
                        setValue("noteUpdate", "");
                    }

                    if task.place.is_some() {
                        setValue("placeUpdate", task.place.unwrap().as_str());
                    } else {
                        setValue("placeUpdate", "");
                    }

                    if task.members.is_some() {
                        setValue("membersUpdate", task.members.unwrap().as_str());
                    } else {
                        setValue("membersUpdate", "");
                    }

                    setValue("deadlineUpdate", task.deadline.as_str());

                    setValue("listUpdate", task.list.to_string().as_str());
                    if task.milestone.is_none() {
                        setValue("milestoneUpdate", "None");
                    }
                    else {
                        setValue("milestoneUpdate", task.milestone.unwrap().to_string().as_str());
                    }
                    Self::Message::Return
                });
                true
            }
            Self::Message::Pass => {
                false
            }
            _ => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.tasks.is_none() {
            let token = self.token.clone().unwrap();
            let id = ctx.props().id;
            let filter = ctx.props().filter.clone();
            ctx.link().send_future(async move {
                let tasks = get_tasks(&token, id, filter).await;
                Self::Message::Update(tasks)
            });
            return html! {};
        }
        let tasks = self.tasks.clone();
        let tasks = tasks.unwrap().into_iter().map(|task| html! {
            <div class="card" style="width: 18rem;">
                <div class="card-body">
                    <h5 class="card-title">{task.name}</h5>
                    <h6 class="card-subtitle mb-2 text-muted">{"Note:"}{task.note.unwrap()}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Place:"}{task.place.unwrap()}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Assigned:"}{task.members.unwrap()}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Deadline:"}{task.deadline}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Points:"}{task.points}</h6>
                    <h6 class="card-subtitle mb-2 text-muted">{"Tags:"}{task.tags}</h6>
                    <SubTasks subtasks={task.subtasks.clone()}/>
                    <Logs id={task.id.unwrap()}/>
                    <button class="btn btn-danger" onclick={ctx.link().callback(move |_: MouseEvent| {Self::Message::Delete(task.id)})}>{"Delete"}</button>
                    <button class="btn btn-primary" onclick={ctx.link().callback(move |_: MouseEvent| {openModal("taskUpdate"); Self::Message::UpdateTask(task.id)})}>{"Update"}</button>
                    <button class="btn btn-primary" onclick={ctx.link().callback(move |_: MouseEvent| {openModal(format!("logs{}", task.id.unwrap()).as_str()); Self::Message::Pass})}>{"Show logs"}</button>
                </div>
            </div>
        });

        //let lists_options = ctx.props().lists.clone();
        //let lists_options = lists_options.unwrap().into_iter().map(|list| html! {
        //                      <ListOptions name={list.name.clone()} board={list.board.clone()} board_type={list.board_type.clone()} id={list.id.clone()} />
        //                });
        html! {
            <>
                <div class="col-xs-6" style="padding-left: 80px;">
                    <h2>{&ctx.props().name}</h2>
                    <button class="btn btn-danger" onclick={ctx.link().callback(move |_: MouseEvent| {Self::Message::DeleteList})}>{"Delete list"}</button>
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
    lists: Option<Vec<List>>,
    pub filter: Option<TaskFilter>,
}

pub struct Board {
    board_type: String,
    board_id: i32,
    lists: Option<Vec<List>>,
    token: Option<String>,
    error: bool,
    filter: Option<TaskFilter>,
    milestones: Option<Vec<Milestone>>,
}

pub enum Msg {
    Submit,
    Res(Result<bool, Error>),
    Update(Result<Vec<List>, Error>),
    UpdateTaskSubmit,
    AddTask,
    Filter,
    Reset,
    UpdateMilestones(Result<Vec<Milestone>, Error>)
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
            error: false,
            filter: None,
            milestones: None
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
                    let res = create_list(
                        &token,
                        List {
                            id: None,
                            name,
                            board,
                            board_type,
                        },
                    )
                    .await;
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
                let deadline = getValue("deadline");
                let subtasks = getValue("subtasks");
                let points = getValue("points").parse::<i32>().unwrap();
                let tags = getValue("tags");
                let done = is_checked("done");
                let milestone = map_result(getValue("milestone").parse::<i32>());
                ctx.link().send_future(async move {
                    let res = create_task(
                        &token,
                        Task {
                            subtasks,
                            deadline,
                            id: None,
                            name,
                            note: Some(note),
                            place: Some(place),
                            members: Some(members),
                            list,
                            points,
                            tags,
                            done,
                            milestone
                        },
                    )
                    .await;
                    Self::Message::Res(res)
                });
                false
            }
            Self::Message::Reset => {
                self.filter = None;
                self.lists = None;
                true
            }
            Self::Message::UpdateTaskSubmit => {
                let token = self.token.clone().unwrap();
                let name = getValue("nameUpdateTask");
                let note = getValue("noteUpdate");
                let place = getValue("placeUpdate");
                let members = getValue("membersUpdate");
                let list = getValue("listUpdate").parse::<i32>().unwrap();
                let id = getValue("idUpdate").parse::<i32>().unwrap();
                let deadline = getValue("deadlineUpdate");
                let subtasks = getValue("subtasksUpdate");
                let points = getValue("pointsUpdate").parse::<i32>().unwrap();
                let tags = getValue("tagsUpdate");
                let done = is_checked("doneUpdate");
                let milestone = map_result(getValue("milestoneUpdate").parse::<i32>());
                ctx.link().send_future(async move {
                    let res = update_task(
                        &token,
                        Task {
                            subtasks,
                            id: Some(id),
                            name,
                            note: Some(note),
                            place: Some(place),
                            members: Some(members),
                            list,
                            deadline,
                            points,
                            tags,
                            done,
                            milestone
                        },
                    )
                    .await;
                    Self::Message::Res(res)
                });
                false
            }
            Self::Message::UpdateMilestones(Ok(milestones)) => {
                self.milestones = Some(milestones);
                true
            }
            Self::Message::Filter => {
                let name = getValue("nameTaskFilter");
                let place = getValue("placeFilter");
                let members = getValue("membersFilter");
                let deadline_start = getValue("deadlineStart");
                let deadline_end = getValue("deadlineEnd");
                let points_min = map_result(getValue("pointsMin").parse::<i32>());
                let points_max = map_result(getValue("pointsMax").parse::<i32>());
                let tags = getValue("tagsFilter");
                let filter = TaskFilter {
                    name,
                    place,
                    members,
                    deadline_start,
                    deadline_end,
                    points_min,
                    points_max,
                    tags
                };
                self.filter = Some(filter);
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
            return html! {};
        }
        if self.lists.is_none() {
            let token = self.token.clone().unwrap();
            let board_id = self.board_id;
            let board_type = self.board_type.clone();
            ctx.link().send_future(async move {
                let lists = get_lists(board_id, board_type, &token).await;
                Self::Message::Update(lists)
            });
            return html! {};
        }
        if self.milestones.is_none() {
            let token = self.token.clone().unwrap();
            let board_type = self.board_type.clone();
            let id = self.board_id.clone();
            ctx.link().send_future(async move {
                let res = get_milestones(id, board_type, &token).await;
                Self::Message::UpdateMilestones(res)
            });
            return html! {}
        }
        let lists = self.lists.clone();
        let lists_clone = lists.clone();
        let filter = self.filter.clone();
        let lists = lists.unwrap().into_iter().map(|list| {
            html! {
                <ListDetails name={list.name} id ={list.id.unwrap()} lists={lists_clone.clone()} filter={filter.clone()}/>
            }
        });
        let lists_options = self.lists.clone();
        let lists_options = lists_options.unwrap().into_iter().map(|list| html! {
                                <ListOptions name={list.name} board={list.board} board_type={list.board_type} id={list.id} />
                            });
        let lists_options_copy = lists_options.clone();
        let milestone_options = self.milestones.clone();
        let milestone_options = milestone_options.unwrap().into_iter().map(|milestone| html! {
            <option value={milestone.id.unwrap().to_string()}>{milestone.name}</option>
        });
        let milestone_options_clone = milestone_options.clone();
        html! {
            <>
            <Navbar />
            <div class="row">
                {for lists}
                <div class="col-xs-6" style="padding-left: 80px;">
                    <h1>{"Add new list"}</h1>
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
                    <button class="btn btn-primary" id="myBtn" onclick={|_: MouseEvent| {openModal("myModal");}} >{"Add task"}</button>
                    <button class="btn btn-primary" id="myBtnFilter" onclick={move |_: MouseEvent| {if filter.clone().is_some() {filter.clone().unwrap().set_filters();} openModal("filterModal");}} >{"Filter tasks"}</button>
                    <button class="btn btn-danger" id="myBtnReset" onclick={ctx.link().callback(|_: MouseEvent| {Msg::Reset})}>{"Reset filters"}</button>
                    <div id="myModal" class="modal">

                    <div class="modal-content">
                        <span class="close btn btn-danger" onclick={|_: MouseEvent| {hideModal("myModal");}}>{"Hide"}</span>
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
                        <div class="form-group">
                            <label for="deadline">{"Deadline:"}</label>
                            <input type="date" class="form-control" id="deadline"/>
                        </div>
                        <div class="form-group">
                            <label for="subtasks">{"Subtasks:"}</label>
                            <input type="text" class="form-control" id="subtasks"/>
                            <small id="subTasksHelp" class="form-text text-muted">{"Subtasks should be seperated by ;"}</small>
                        </div>
                        <div class="form-group">
                            <label for="team">{"Choose list:"}</label>
                                <select id="list" name="team">
                                    {for lists_options}
                                </select>
                        </div>
                        <div class="form-group">
                            <label for="milestone">{"Choose milestone:"}</label>
                                <select id="milestone">
                                    <option value="None">{"None"}</option>
                                    {for milestone_options}
                                </select>
                        </div>
                        <div class="form-group">
                            <input type="checkbox" id="done" name="Done" value="yes"/>  
                            <label for="done">{"Done:"}</label>
                        </div>
                        <div class="form-group">
                            <label for="points">{"Function points:"}</label>
                            <input type="number" class="form-control" id="points" min="0"/>
                        </div>
                        <div class="form-group">
                            <label for="tags">{"Tags:"}</label>
                            <input type="text" class="form-control"  id="tags"/>
                            <small id="TagsHelp" class="form-text text-muted">{"Tags should be seperated by ;"}</small>
                        </div>
                        <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Msg::AddTask})}>{"Submit"}</button>
                    </form>
                    </div>

                    </div>
                </div>
                <div id="taskUpdate" class="modal">

                <div class="modal-content">
                    <span class="close btn btn-danger" onclick={|_: MouseEvent| {hideModal("taskUpdate");}}>{"Hide"}</span>
                    <form>
                    <div class="form-group">
                        <label for="name">{"name"}</label>
                        <input type="text" class="form-control" id="nameUpdateTask" aria-describedby="usernameHelp" placeholder="Enter new task name"/>
                    </div>
                    <div class="form-group">
                        <label for="note">{"note"}</label>
                        <input type="text" class="form-control" id="noteUpdate" aria-describedby="usernameHelp" placeholder="Enter note"/>
                    </div>
                    <div class="form-group">
                        <label for="place">{"place"}</label>
                        <input type="text" class="form-control" id="placeUpdate" aria-describedby="usernameHelp" placeholder="Enter place"/>
                    </div>
                    <div class="form-group">
                        <input type="hidden" class="form-control" id="idUpdate" aria-describedby="usernameHelp" placeholder="Enter place"/>
                    </div>
                    <div class="form-group">
                        <label for="members">{"Assigned people"}</label>
                        <input type="text" class="form-control" id="membersUpdate" aria-describedby="usernameHelp" placeholder="Enter assigned people"/>
                        <small id="emailHelp" class="form-text text-muted">{"Assigned people should be seperated by ;"}</small>
                    </div>
                    <div class="form-group">
                            <label for="deadlineUpdate">{"Deadline:"}</label>
                            <input type="date" class="form-control" id="deadlineUpdate"/>
                    </div>
                    <div class="form-group">
                            <label for="subtasksUpdate">{"Subtasks:"}</label>
                            <input type="text" class="form-control" id="subtasksUpdate"/>
                            <small id="subTasksHelpUpdate" class="form-text text-muted">{"Subtasks should be seperated by ;"}</small>
                    </div>
                    <div class="form-group">
                            <label for="pointsUpdate">{"Function points:"}</label>
                            <input type="number" class="form-control" id="pointsUpdate" min="0"/>
                        </div>
                    <div class="form-group">
                    <label for="team">{"Choose list:"}</label>
                        <select id="listUpdate" name="team">
                            {for lists_options_copy}
                        </select>
                    </div>
                    <div class="form-group">
                            <label for="milestoneUpdate">{"Choose milestone:"}</label>
                                <select id="milestoneUpdate">
                                <option value="None">{"None"}</option>
                                    {for milestone_options_clone}
                                </select>
                    </div>
                    <div class="form-group">
                            <input type="checkbox" id="doneUpdate" name="Done" value="yes"/>  
                            <label for="doneUpdate">{"Done:"}</label>
                    </div>
                    <div class="form-group">
                            <label for="tagsUpdate">{"Tags:"}</label>
                            <input type="text" class="form-control"  id="tagsUpdate"/>
                            <small id="TagsHelp" class="form-text text-muted">{"Tags should be seperated by ;"}</small>
                        </div>
                    <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Self::Message::UpdateTaskSubmit})}>{"Submit"}</button>
                </form>
                </div>
                </div>
                <div id="filterModal" class="modal">

                <div class="modal-content">
                    <span class="close btn btn-danger" onclick={|_: MouseEvent| {hideModal("filterModal");}}>{"Hide"}</span>
                    <form>
                    <div class="form-group">
                        <label for="nameTaskFilter">{"name contains"}</label>
                        <input type="text" class="form-control" id="nameTaskFilter" aria-describedby="usernameHelp" placeholder="Enter task name"/>
                    </div>
                    <div class="form-group">
                        <label for="placeFilter">{"place contains"}</label>
                        <input type="text" class="form-control" id="placeFilter" aria-describedby="usernameHelp" placeholder="Enter place"/>
                    </div>
                    <div class="form-group">
                        <label for="membersFilter">{"Assigned people contains"}</label>
                        <input type="text" class="form-control" id="membersFilter" aria-describedby="usernameHelp" placeholder="Enter assigned people"/>
                        <small id="membersHelpFilter" class="form-text text-muted">{"Assigned people should be seperated by ;"}</small>
                    </div>
                    <div class="form-group">
                        <label for="deadlineStart">{"Deadline start:"}</label>
                        <input type="date" class="form-control" id="deadlineStart"/>
                        <small id="statrtHelp" class="form-text text-muted">{"Leave empty for no filter"}</small>
                    </div>
                    <div class="form-group">
                        <label for="deadlineEnd">{"Deadline end:"}</label>
                        <input type="date" class="form-control" id="deadlineEnd"/>
                        <small id="endHelp" class="form-text text-muted">{"Leave empty for no filter"}</small>
                    </div>
                    <div class="form-group">
                        <label for="pointsMin">{"Points min:"}</label>
                        <input type="number" class="form-control" id="pointsMin"/>
                        <small id="minHelp" class="form-text text-muted">{"Leave empty for no filter"}</small>
                    </div>
                    <div class="form-group">
                        <label for="pointsMax">{"Points max:"}</label>
                        <input type="number" class="form-control" id="pointsMax"/>
                        <small id="maxHelp" class="form-text text-muted">{"Leave empty for no filter"}</small>
                    </div>
                    <div class="form-group">
                        <label for="tags">{"Tags contains:"}</label>
                        <input type="text" class="form-control"  id="tagsFilter"/>
                        <small id="TagsHelp" class="form-text text-muted">{"Tags should be seperated by ;"}</small>
                    </div>
                    <button type="submit" class="btn btn-primary" onclick={ctx.link().callback(|e: MouseEvent| {e.prevent_default(); Msg::Filter})}>{"Submit"}</button>
                </form>
                </div>

                </div>
                <div class="col-xs-6 vl"></div>
                <MilestoneList id={self.board_id.clone()} board_type={self.board_type.clone()} milestones={self.milestones.clone()}/>  
            </div>
            </>
        }
    }
}
