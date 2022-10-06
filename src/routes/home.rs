#[get("/")]
pub fn index() -> &'static str {
    "Welcome Home"
}