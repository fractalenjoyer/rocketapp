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
        .mount(
            "/",
            routes![
                index,
                count,
                create,
                upload,
                db_ops::get_users,
                db_ops::get_user,
                test,
                test2
            ],
        )
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
        .persist_to(format!("static/content/{img_path}",))
        .await
        .ok()?;
    db_ops::create_post(
        db,
        db_ops::Post {
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
async fn register(db: Connection<db_ops::MyDatabase>, user: Form<User>) -> Option<Redirect> {
    db_ops::create_user(db, user.into_inner()).ok()?;
    Some(Redirect::to("/"))
}

#[get("/<password>/<hash>")]
fn test2(password: String, hash: String) -> Option<String> {
    Some(db_ops::verify_password(password, hash).to_string())
}
