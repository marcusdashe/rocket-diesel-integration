#![allow(unused)]

use diesel::prelude::*;
use crate::*;



#[delete("/delete/<keyword>")]
pub fn del_post(keyword: &str) -> String {
    use crate::schema::posts::dsl::*;
    let target = keyword;
    let pattern = format!("%{}%", target);
    let conn = &mut crate::establish_connection();
    
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
    .execute(conn)
    .expect("Error deleteing posts");

    format!("Deleted {} posts", num_deleted)
}
