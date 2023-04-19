#[macro_use]
extern crate rocket;

use std::sync::RwLock;

use rocket::fs::FileServer;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;

use rocket_db_pools::{Connection, Database};

mod db_ops;

use rocket_dyn_templates::{context, Template};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db_ops::MyDatabase::init())
        .attach(Template::fairing())
        .register("/", catchers![not_found])
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index, count, create, upload])
}

#[get("/")]
async fn index(db: Connection<db_ops::MyDatabase>) -> Option<Template> {
    let posts = db_ops::get_posts(db).await.ok()?;
    Some(Template::render(
        "index",
        context! {
            title: "Hello!",
            style: "index.css",
            posts
        },
    ))
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
async fn upload(db: Connection<db_ops::MyDatabase>, mut post: Form<Post<'_>>) -> Option<Redirect> {
    println!("Title: {}", post.title);
    let img_path = format!("{}.png", Uuid::new_v4());
    post.image
        .persist_to(format!("static/content/{}", img_path))
        .await
        .ok()?;
    db_ops::create_post(
        db,
        db_ops::Post {
            id: None,
            title: post.title.clone(),
            body: "".to_string(),
            image: img_path,
        },
    )
    .await
    .ok()?;
    Some(Redirect::to("/upload"))
}
