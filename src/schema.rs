// @generated automatically by Diesel CLI.

diesel::table! {
    history (id) {
        id -> Int4,
        chat_id -> BigInt,
        chat_username -> Text,
        hiss -> Bool,
        image -> Int4,
        by_command -> Bool,
        chat_private -> Bool,
        datetime -> Timestamp,
    }
}

diesel::table! {
    banned_imgs (name) {
        name -> Int4,
    }
}