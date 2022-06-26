use rocket::local::blocking::Client;

use super::*;

fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                get_milestones,
                milestone_create,
                get_logs,
                delete_list,
                update_team,
                update_private,
                update_task,
                get_tasks,
                filter_tasks,
                delete_task,
                delete_team_board,
                login,
                register,
                private_board,
                team_create,
                team_board,
                owned,
                get_private_boards,
                get_team_boards,
                delete_private,
                new_list,
                get_list,
                create_task,
                get_task,
                timer_create,
                timer_delete,
                timer_update,
                get_timers
            ],
        )
        .attach(Connection::fairing())
}

fn get_test_user() -> Credentials {
    Credentials {
        username: "test".to_string(),
        password: "test".to_string(),
    }
}

fn login(client: &Client, user: &Credentials) -> TokenResponse {
    client
        .post("/login")
        .json(user)
        .dispatch()
        .into_json::<TokenResponse>()
        .unwrap()
}

#[test]
fn test_register() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid `Rocket`");

    let user = get_test_user();
    let _ = client.post("/register").json(&user).dispatch();

    let response = login(&client, &user);
    assert!(response.success);
}

#[test]
fn test_private_board_creation() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid `Rocket`");

    let user = get_test_user();
    let _ = client.post("/register").json(&user).dispatch();
    let token = login(&client, &user);
    assert!(token.success);

    let board = PrivateBoardData {
        name: "Ala ma kota".to_string(),
    };

    client
        .post("/private_board/create")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .json(&board)
        .dispatch();

    let boards = client
        .get("/private_board/get")
        .header(rocket::http::Header::new("Authorization", token.token))
        .dispatch()
        .into_json::<Vec<PrivateBoard>>()
        .unwrap();

    let filter = boards
        .into_iter()
        .filter(|board| board.name == "Ala ma kota".to_string())
        .count();
    assert!(filter > 0);
}

#[test]
fn test_team_board_creation() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid `Rocket`");

    let user = get_test_user();
    let _ = client.post("/register").json(&user).dispatch();
    let token = login(&client, &user);
    assert!(token.success);

    let team = TeamData {
        name: "Ala ma kota".to_string(),
        members: "test".to_string(),
    };

    client
        .post("/team/create")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .json(&team)
        .dispatch();

    let teams = client
        .get("/owned")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .dispatch()
        .into_json::<Vec<Team>>()
        .unwrap();
    let team = teams.get(0).unwrap();

    let board = TeamBoardData {
        owner: team.id.unwrap(),
        name: "Ala ma kota".to_string(),
    };

    client
        .post("/team_board/create")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .json(&board)
        .dispatch();

    let boards = client
        .get("/team_board/get")
        .header(rocket::http::Header::new("Authorization", token.token))
        .dispatch()
        .into_json::<Vec<TeamBoard>>()
        .unwrap();

    let filter = boards
        .into_iter()
        .filter(|board| board.name == "Ala ma kota".to_string())
        .count();
    assert!(filter > 0);
}

#[test]
fn test_milestone_creation() {
    test_private_board_creation();
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid `Rocket`");

    let user = get_test_user();
    let _ = client.post("/register").json(&user).dispatch();
    let token = login(&client, &user);
    assert!(token.success);

    let milestone = Milestone {
        id: None,
        name: "ala ma kota".to_string(),
        board_id: 1,
        board_type: "private".to_string(),
    };

    client
        .post("/milestone/create")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .json(&milestone)
        .dispatch();
    let milestones = client
        .get("/milestone/get/1/private")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .dispatch()
        .into_json::<Vec<MilestoneResponse>>()
        .unwrap();
    let milestone = milestones.get(0).unwrap();
    assert_eq!(milestone.board_id, 1);
    assert_eq!(milestone.board_type, "private");
    assert_eq!(milestone.name, "ala ma kota");
    assert_eq!(milestone.done, 0);
    assert_eq!(milestone.total, 0);
}

#[test]
fn test_timer() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid `Rocket`");

    let user = get_test_user();
    let _ = client.post("/register").json(&user).dispatch();
    let token = login(&client, &user);
    assert!(token.success);

    let timer = TimerData {
        name: "Ala ma kota".to_string(),
    };

    client
        .post("/timer/create")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .json(&timer)
        .dispatch();

    let timers = client
        .get("/timers/get")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .dispatch()
        .into_json::<Vec<Timer>>()
        .unwrap();
    let timer = timers.get(0).unwrap();
    assert_eq!(timer.status, "active");

    client
        .get(format!("/timer/update/{}", timer.id.unwrap()))
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .dispatch();
    let timers = client
        .get("/timers/get")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .dispatch()
        .into_json::<Vec<Timer>>()
        .unwrap();
    let timer = timers.get(0).unwrap();
    assert_eq!(timer.status, "stopped");

    client
        .get(format!("/timer/delete/{}", timer.id.unwrap()))
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .dispatch();
    let timers = client
        .get("/timers/get")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .dispatch()
        .into_json::<Vec<Timer>>()
        .unwrap();
    assert_eq!(timers.into_iter().filter(|t| t.id == timer.id).count(), 0);
}

#[test]
fn test_list_and_task() {
    test_team_board_creation();
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid `Rocket`");

    let user = get_test_user();
    let _ = client.post("/register").json(&user).dispatch();
    let token = login(&client, &user);
    assert!(token.success);

    let boards = client
        .get("/team_board/get")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .dispatch()
        .into_json::<Vec<TeamBoard>>()
        .unwrap();
    let board_id = boards.get(0).unwrap().id.unwrap();

    let list = List {
        id: None,
        name: "Ala ma kota".to_string(),
        board_type: "team".to_string(),
        board: board_id,
    };

    client
        .post("/new_list")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .json(&list)
        .dispatch();
    let lists = client
        .get(format!("/list/team/{}", board_id))
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .dispatch()
        .into_json::<Vec<List>>()
        .unwrap();
    assert_eq!(lists.get(0).unwrap().name, "Ala ma kota");

    let task = Task {
        id: None,
        name: "name".to_string(),
        list: lists.get(0).unwrap().id.unwrap(),
        note: Some("note".to_string()),
        place: Some("place".to_string()),
        members: Some("test".to_string()),
        deadline: "".to_string(),
        subtasks: "".to_string(),
        points: 0,
        tags: "".to_string(),
        done: 0,
        milestone: None,
    };

    client
        .post("/task/create")
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .json(&task)
        .dispatch();
    let tasks = client
        .get(format!("/task/get/{}", task.list))
        .header(rocket::http::Header::new(
            "Authorization",
            token.token.clone(),
        ))
        .dispatch()
        .into_json::<Vec<Task>>()
        .unwrap();
    let task = tasks.get(0).unwrap();
    assert_eq!("name", task.name);
    assert_eq!("note", task.note.as_ref().unwrap());
    assert_eq!("test", task.members.as_ref().unwrap());
    assert_eq!(0, task.points);
    assert_eq!(0, task.done);
    assert!(task.milestone.is_none());
}
