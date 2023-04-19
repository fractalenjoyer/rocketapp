#[macro_use]
extern crate rocket;

use std::sync::RwLock;

use rocket::fs::FileServer;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;

// use rocket_db_pools::{Database};

use rocket_dyn_templates::{context, Template};

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .attach(db_ops::MyDatabase::init())
        .attach(Template::fairing())
        .register("/", catchers![not_found])
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index, count, create, upload])
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Hello!",
            style: "index.css"
        },
    )
}

#[get("/upload")]
fn create() -> Template {
    Template::render(
        "upload",
        context! {
            title: "Upload",
            style: "upload.css"
        },
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

static COUNT: RwLock<u32> = RwLock::new(0);

#[get("/count")]
fn count() -> Option<RawHtml<String>> {
    let mut current = COUNT.write().ok()?;
    *current += 1;

    Some(RawHtml(
        maud::html!(
            h1 { "Count: " (*current) }
        )
        .into_string(),
    ))
}

use rocket::form::Form;
use rocket::fs::TempFile;
use uuid::Uuid;

#[derive(FromForm)]
struct Post<'r> {
    title: String,
    image: TempFile<'r>,
}

#[post("/upload", data = "<post>")]
async fn upload(mut post: Form<Post<'_>>) -> Option<Redirect> {
    println!("Title: {}", post.title);
    post.image
        .persist_to(format!("static/content/{}.png", Uuid::new_v4()))
        .await
        .ok()?;
    Some(Redirect::to("/upload"))
}
