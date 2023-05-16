#[macro_use]
extern crate rocket;

use rocket::{fs::FileServer, response::content::RawHtml,};
// use rocket::{fairing, Rocket, Build}; // to add migrations at some point
use rocket_db_pools::Database;
use rocket_dyn_templates::{context, Template};

mod api;
mod auth;
mod database;
mod files;
mod pages;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::MyDatabase::init())
        .attach(Template::fairing())
        .register("/", catchers![not_found, unauthorized])
        .mount("/static", FileServer::from("static"))
        .mount(
            "/",
            routes![
                pages::index,
                pages::create,
                pages::register,
                pages::login,
                pages::profile,
                pages::post,
            ],
        )
        .mount(
            "/api",
            routes![
                api::whoami,
                api::userdetails,
                api::deletepost,
                api::create_post,
                api::register_user,
                api::login_user,
                api::logout,
                api::comment
            ],
        )
}

#[catch(401)]
fn unauthorized() -> RawHtml<String> {
    RawHtml(
        maud::html!(
            h1 { "Login to view this page" }
            a href="/login" { "Login" }
        )
        .into_string(),
    )
}

#[catch(404)]
fn not_found() -> Template {
    Template::render(
        "404",
        context! {
            title: "404",
            style: "404.css"
        },
    )
}