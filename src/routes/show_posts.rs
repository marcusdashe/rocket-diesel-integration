#![allow(unused)]

use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::models::{Post};

#[get("/get-posts?<unpublished>")]
pub fn get_posts(unpublished: bool)-> Json<Vec<Post>>{
    use crate::schema::posts::dsl::*;

    let conn = &mut crate::establish_connection();

    let results = if(unpublished) {
        posts.filter(published.eq(false))
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts")
    } else {
        posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts")
    };

    
    return Json(results);
}