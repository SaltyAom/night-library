use actix_web::get;

#[get("/")]
pub async fn index() -> String {
    "Hello World".to_owned()
}
