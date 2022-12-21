// @generated automatically by Diesel CLI.

diesel::table! {
    bfp (id) {
        id -> Int4,
        bfp_hash -> Varchar,
        user_agent -> Varchar,
    }
}
