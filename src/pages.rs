use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::auth;
use crate::database;

#[get("/")]
pub async fn index(db: Connection<database::MyDatabase>) -> Option<Template> {
    let (posts, posters) = database::get_posts(db).await.ok()?;
    println!("{:?}", posters);
    Some(Template::render(
        "index",
        context! {
            title: "Hello!",
            style: "index.css",
            posts,
            posters
        },
    ))
}

#[get("/upload")]
pub fn create(_user: auth::User) -> Template {
    Template::render(
        "upload",
        context! {
            title: "Upload",
            style: "upload.css"
        },
    )
}

#[get("/register")]
pub fn register() -> Template {
    Template::render(
        "register",
        context! {
            title: "Register",
            style: "login.css"
        },
    )
}

#[get("/login")]
pub fn login() -> Template {
    Template::render(
        "login",
        context! {
            title: "Login",
            style: "login.css"
        },
    )
}

#[get("/profile")]
pub async fn profile(_user: auth::User) -> Option<Template> {
    Some(Template::render(
        "profile",
        context! {
            title: "Profile",
            style: "profile.css",
        },
    ))
}

#[get("/post/<id>")]
pub async fn post(id: i32, db: Connection<database::MyDatabase>) -> Option<Template> {
    let (post, poster, comments) = database::get_post_by_id(db, id).await.ok()?;
    Some(Template::render(
        "post",
        context! {
            title: "Post",
            style: "post.css",
            post,
            poster,
            comments
        },
    ))
}

#[get("/user/<username>")]
pub async fn user(username: String, db: Connection<database::MyDatabase>) -> Option<Template> {
    let (posts, posters) = database::get_posts_by_user(db, username).await.ok()?;
    Some(Template::render(
        "user",
        context! {
            title: "User",
            style: "user.css",
            posts,
            posters
        },
    ))
}