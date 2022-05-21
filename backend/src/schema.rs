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