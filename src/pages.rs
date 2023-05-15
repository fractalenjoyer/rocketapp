use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::auth;
use crate::database;
use crate::database::Post;

#[get("/")]
pub async fn index(db: Connection<database::MyDatabase>) -> Option<Template> {
    let (posts, db) = database::get_posts(db).await.ok()?;
    let posts = database::get_users_by_posts(db, &posts)
        .await
        .ok()?
        .iter()
        .map(|user| user.username.clone())
        .zip(posts.iter())
        .collect::<Vec<(String, &Post)>>();

    Some(Template::render(
        "index",
        context! {
            title: "Hello!",
            style: "index.css",
            posts,
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
