#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::response::content::RawHtml;

use rocket_db_pools::{Database};

use rocket_dyn_templates::{context, Template};

use maud::html;

mod db_ops;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db_ops::MyDatabase::init())
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index, db_ops::get_user, hello, db_ops::get_users, new, upload])
}


#[get("/")]
fn index() -> RawHtml<String> {
    RawHtml(html! {
        head {
            link rel="stylesheet" href="/static/styles/style.css" {}
        }
        img src="https://upload.wikimedia.org/wikipedia/commons/thumb/b/b2/Hamburger_icon.svg/2048px-Hamburger_icon.svg.png" {}
        div {
            h1 { "Hello, rocket!" }
            p { "Pretty neat stuff" }
        }
    }.into_string())
}

#[get("/new")]
fn new() -> Template {
    Template::render("create", context! {
            title: "Create a new user",
        },
    )
}

#[get("/hello/<name>")]
fn hello(name: &str) -> Template {
    Template::render("index", context! {
            name: name,
            title: "Hello!",
        },
    )
}

use rocket::form::Form;
use rocket::fs::TempFile;

#[derive(FromForm)]
struct Post<'r> {
    title: String,
    image: TempFile<'r>
}

#[post("/upload", data = "<post>")]
async fn upload(mut post: Form<Post<'_>>) -> Option<Redirect> {
    println!("Text: {}", post.title);
    post.image.copy_to("static/images/damn.png").await.ok()?;
    Some(Redirect::to("/new"))
}

