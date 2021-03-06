table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Varchar,
        password -> Varchar,
    }
}

table! {
    private_board (id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        owner -> Integer,
    }
}

table! {
    team (id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        owner -> Integer,
    }
}

table! {
    team_user (id) {
        id -> Nullable<Integer>,
        team -> Integer,
        user -> Integer,
    }
}

table! {
    team_board (id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        owner -> Integer,
    }
}

table! {
    list(id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        board -> Integer,
        board_type -> Varchar,
    }
}

table! {
    task(id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        list -> Integer,
        note -> Nullable<Varchar>,
        place -> Nullable<Varchar>,
        members -> Nullable<Varchar>,
        deadline -> Varchar,
        subtasks -> Varchar,
        points -> Integer,
        tags -> Varchar,
        done -> Integer,
        milestone -> Nullable<Integer>,
    }
}

table! {
    milestone(id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        board_id -> Integer,
        board_type -> Varchar,
    }
}

table! {
    log(id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        list -> Integer,
        note -> Nullable<Varchar>,
        place -> Nullable<Varchar>,
        members -> Nullable<Varchar>,
        timestamp -> Varchar,
        action -> Varchar,
        task_id -> Integer,
        deadline -> Varchar,
        subtasks -> Varchar,
        points -> Integer,
        tags -> Varchar,
    }
}

table! {
    timer(id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        user_id -> Integer,
        status -> Varchar,
        time -> Integer,
        start -> Nullable<Integer>,
    }
}
