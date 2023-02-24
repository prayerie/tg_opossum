use std::time::SystemTime;

use diesel::{prelude::*, sql_types::{Int4, Bool, Text}, dsl};
use crate::schema::{history, banned_imgs};

#[derive(Queryable)]
pub struct History {
    pub id: i32,
    pub chat_id: i64,
    pub chat_username: String,
    pub hiss: bool,
    pub image: i32,
    pub by_command: bool,
    pub chat_private: bool,
    pub datetime: SystemTime
}

#[derive(Queryable, Debug)]
pub struct Banned {
    pub name: i32,
}


#[derive(Insertable)]
#[diesel(table_name = history)]
pub struct InsertHist<'a> {
    pub chat_id: &'a i64,
    pub chat_username: &'a str,
    pub hiss: &'a bool,
    pub image: &'a i32,
    pub by_command: &'a bool,
    pub chat_private: &'a bool,
    pub datetime: &'a dsl::now,
}

#[derive(Insertable)]
#[diesel(table_name = banned_imgs)]
pub struct InsertBanned<'a> {
    pub name: &'a i32,
}