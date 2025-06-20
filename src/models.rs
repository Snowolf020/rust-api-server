use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    id: i32,
    title: String,
    content: String,
    user_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
struct Comment {
    id: i32,
    content: String,
    post_id: i32,
    user_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}