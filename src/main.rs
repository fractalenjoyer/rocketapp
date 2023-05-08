#[macro_use]
extern crate rocket;

use std::sync::RwLock;

use rocket::fs::FileServer;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;

use rocket_db_pools::{Connection, Database};

mod database;
mod auth;

use rocket_dyn_templates::{context, Template};

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
                index,
                count,
                create,
                upload,
                database::get_users,
                database::get_user,
            ],
        )
}

#[get("/")]
async fn index(db: Connection<database::MyDatabase>) -> Option<Template> {
    let posts = database::get_posts(db).await.ok()?;
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
async fn upload(db: Connection<database::MyDatabase>, mut post: Form<Post<'_>>) -> Option<Redirect> {
    println!("Title: {}", post.title);
    let img_path = format!("{}.png", Uuid::new_v4());
    post.image
        .persist_to(format!("static/content/{img_path}",))
        .await
        .ok()?;
    database::create_post(
        db,
        database::Post {
            id: None,
            title: post.title.clone(),
            body: String::new(),
            image: img_path,
        },
    )
    .await
    .ok()?;
    Some(Redirect::to("/upload"))
}

#[derive(FromForm)]
struct User {
    username: String,
    password: String,
}

#[post("/register", data = "<user>")]
async fn register(db: Connection<database::MyDatabase>, user: Form<User>) -> Option<Redirect> {
    database::create_user(
        db,
        user.username.clone(),
        user.password.clone(),
    );
    Some(Redirect::to("/"))
}
