extern crate mergerd;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    mergerd::rocket().launch().await?;

    Ok(())
}
