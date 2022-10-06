use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::models::{Post};

#[put("/publish/<id>")]
pub fn publish(id: i32)-> Json<Post>{
    use crate::schema::posts::dsl::{posts, published};
    
    let conn = &mut crate::establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .unwrap();

    Json(post)
}