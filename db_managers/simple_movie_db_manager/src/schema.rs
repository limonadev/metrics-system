table! {
    movies (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    ratings (id) {
        id -> Int4,
        user_id -> Int4,
        movie_id -> Int4,
        rating -> Float8,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
    }
}

joinable!(ratings -> movies (movie_id));
joinable!(ratings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    movies,
    ratings,
    users,
);
