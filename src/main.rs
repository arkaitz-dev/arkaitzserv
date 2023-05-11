//! ### Spaserver
//! 
//! As the name suggests, it's a lightweight web server aimed at serving SPA web pages.
//! It simply performs a redirection of any URL (relative to your domain, of course) to
//! "/#/whateveryoutype". By doing this, if you properly process the new URL in your routes,
//! you'll be able to correctly deal with hand written URLs.
//! 
//! In order to give you just a little bit of freedom, you can set the following environment
//! variables:
//! 
//! * ROOT_DIR: It defaults to "public". It's where your index.html file lives, relative to 
//! where spaserver is executed, and the base of other folders you need for your SPA.
//! 
//! * ASSETS_DIR: It defaults to "assets". It's where you can find style files, icons and other
//! stuff.
//! 
//! * API_URL: In case you use an external API (for example for database lookups, or 
//! authentication, etc), every time you call "/api", it will be rewritten to this URL. Defaults
//! to the "standard" rewrite policy previously explained.
//! 
//! As you can see, this crate it's not intended for any complex configuration. Of course, it
//! comes with absolutely no guarantee, so use it at your own risk.

#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use std::env;
use std::path::PathBuf;

#[get("/<path..>", rank = 3)]
fn rewrite(path: PathBuf) -> Redirect {
    Redirect::to(format!("/#/{}", path.to_string_lossy()))
}

#[get("/<path..>", rank = 4)]
fn api_get(path: PathBuf) -> Redirect {
    let path_str: String = path.to_string_lossy().to_string();
    match env::var("API_URL") {
        Ok(api_url) => Redirect::permanent(format!("{}/{}", api_url, path_str)),
        _ => Redirect::permanent(format!("/#/{}", path_str)),
    }
}

#[launch]
fn rocket() -> _ {
    let root = env::var("ROOT_DIR").unwrap_or("public".to_string());
    let assets = env::var("ASSETS_DIR").unwrap_or("assets".to_string());

    rocket::build()
        .mount("/", FileServer::from(&root).rank(2))
        .mount(format!("/{}",assets), FileServer::from(format!("{}/{}", root, assets)).rank(1))
        .mount("/", routes![rewrite])
        .mount("/api", routes![api_get])
}
