use rocket::fs::NamedFile;

use rocket::response::status::NotFound;

pub mod wallet_rpc;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("html/antibot/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
