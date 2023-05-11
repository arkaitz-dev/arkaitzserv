#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use std::env;
use std::path::PathBuf;

// #[get("/")]
// async fn index() -> Option<NamedFile> {
//     NamedFile::open("static/index.html").await.ok()
// }

#[get("/<path..>", rank = 3)]
fn rewrite(path: PathBuf) -> Redirect {
    Redirect::to(format!("/#/{}", path.to_string_lossy()))
}

#[get("/<path..>", rank = 4)]
fn api_get(path: PathBuf) -> Redirect {
    let path_str: String = path.to_string_lossy().to_string();
    match env::var("API_URL") {
        Ok(api_url) => Redirect::permanent(format!("{}{}", api_url, path_str)),
        _ => Redirect::permanent(format!("/#/{}", path_str)),
    }
}

#[launch]
fn rocket() -> _ {
    let root = env::var("ROOT_DIR").unwrap_or("public".to_string());
    rocket::build()
        .mount("/", FileServer::from(&root).rank(2))
        .mount("/assets", FileServer::from(format!("{}/assets", root)).rank(1))
        .mount("/", routes![rewrite])
        .mount("/api", routes![api_get])
}
