
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::form::{ Form };
use dotenvy::dotenv;
use std::env;
use crate::models::{NewPost, Post};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(FromForm)]
pub struct CreatePost<'a> {
    title: &'a str,
    body: &'a str
}

#[post("/create-post", data = "<post>")]
pub fn create(post: Form<CreatePost<'_>>) -> Json<String> {
        use crate::schema::posts;

        let new_post = NewPost {
            title: post.title,
            body: post.body,
        };

        let conn = &mut crate::establish_connection();

        diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(conn)
        .expect("Error saving new post");

        Json(format!("Title: {}, Writeups: {}", post.title, post.body))
        
}
