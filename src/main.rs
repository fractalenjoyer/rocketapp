#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_db_pools::Database;
use rocket_dyn_templates::{context, Template};

mod database;
mod auth;
mod files;
mod pages;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::MyDatabase::init())
        .attach(Template::fairing())
        .register("/", catchers![not_found])
        .mount("/static", FileServer::from("static"))
        .mount(
            "/",
            routes![
                pages::index,
                pages::create_post,
                pages::create,
                pages::register,
                pages::register_user,
                pages::login,
                pages::login_user,
            ],
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
