#[macro_use]
extern crate rocket;

use rocket::{fs::FileServer, response::{content::RawHtml, Redirect},};
// use rocket::{fairing, Rocket, Build}; // to add migrations at some point
use rocket_db_pools::Database;
use rocket_dyn_templates::{context, Template};

mod api;
mod auth;
mod database;
mod files;
mod pages;

/// Initializes the rocket instance
#[launch]
fn rocket() -> _ {
    rocket::build()
        // attaches the database to the rocket instance
        // enables the use of the database in the routes
        // as a request guard
        .attach(database::MyDatabase::init()) 
        // attaches Template fairing to the rocket instance
        // enables the use of templates in the routes
        .attach(Template::fairing())
        // register catchers for 404 and 401 errors
        .register("/", catchers![not_found, unauthorized, internal_error])
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
                pages::user
            ],
        )
        .mount(
            "/api",
            routes![
                api::whoami,
                api::deletepost,
                api::deletecomment,
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

#[catch(500)]
fn internal_error() -> Redirect {
    Redirect::to("/")
}