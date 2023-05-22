/// This file contains all the routes for the website that return a Template
/// The routes are mounted in src\main.rs
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::auth;
use crate::database;

/// Index page
/// gets posts from the database and renders them
#[get("/")]
pub async fn index(db: Connection<database::MyDatabase>) -> Option<Template> {
    let (posts, posters) = database::get_posts(db).await.ok()?;
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

/// Upload page
/// renders the upload page
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

/// Register page
/// renders the register page
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

/// Login page
/// renders the login page
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

/// Profile page
/// renders a user's profile page
/// currently only shows a sign out link
/// TODO: add more functionality
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

/// Post page
/// renders a post with the given id
/// also renders the comments for the post
#[get("/post/<id>")]
pub async fn post(
    id: i32,
    db: Connection<database::MyDatabase>,
) -> Option<Template> {
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

/// User page
/// renders a user's profile page
/// shows the 20 most recent posts by the user
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