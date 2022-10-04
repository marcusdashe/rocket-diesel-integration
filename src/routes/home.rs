#[get("/home")]
pub fn index() -> &'static str {
    "Welcome Home"
}