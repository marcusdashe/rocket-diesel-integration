use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::models::{Post};

#[get("/get-posts")]
pub fn get_posts()-> Json<Vec<Post>>{
    use crate::schema::posts::dsl::*;

    let conn = &mut crate::establish_connection();

    let results = posts
    .filter(published.eq(true))
    .limit(5)
    .load::<Post>(conn)
    .expect("Error loading posts");

    
        return Json(results);
}