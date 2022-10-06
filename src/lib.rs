
#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

use diesel::pg::PgConnection; 
use diesel::prelude::*;

use rocket::http::Method;

extern crate rocket_cors;
use rocket_cors::{AllowedOrigins, CorsOptions};


#[macro_use]
extern crate diesel;


use dotenvy::dotenv;
use std::env;


pub mod routes;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "Error",
        "reason": "Resource was not found"
    })
}



// fn cors_fairing() -> Cors {
//     CorsOptions::default()
//         .to_cors()
//         .expect("Cors fairing cannot be created")
// }

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();

    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch, Method::Delete, Method::Put]
        .into_iter()
        .map(From::from)
        .collect(),
    )
    .allow_credentials(true);

    rocket::build()
    .manage(cors.to_cors())
    .mount("/api", routes![
                        routes::home::index,
                        routes::create_post::create,
                        routes::show_posts::get_posts,
                        routes::publish_post::publish,
                        routes::delete_post::del_post
                        ])
    .register("/", catchers![not_found])
    
}