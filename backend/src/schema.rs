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

