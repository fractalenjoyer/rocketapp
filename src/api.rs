/// This file contains all API routes
/// The routes are mounted in src\main.rs
use rocket::form::Form;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_db_pools::Connection;

use crate::auth;
use crate::database;
use crate::files;

#[get("/whoami")]
pub fn whoami(user: auth::User) -> String {
    format!("Hello, {}!", user.username)
}

/// Deletes a post with the given id
/// args are automatically parsed from the request body
/// redirects to the index page
/// only works if the user is the owner of the post
#[get("/deletepost/<id>")]
pub async fn deletepost(
    id: i32,
    user: auth::User,
    db: Connection<database::MyDatabase>,
) -> Result<Redirect, Status> {
    database::delete_post(db, user.id, id)
        .await
        .map_err(|_| Status::Unauthorized)?;
    Ok(Redirect::to("/"))
}

/// Deletes a comment with the given id
/// args are automatically parsed from the request body
/// redirects to the index page
/// only works if the user is the owner of the comment
#[get("/deletecomment/<id>")]
pub async fn deletecomment(
    id: i32,
    user: auth::User,
    db: Connection<database::MyDatabase>,
) -> Result<Redirect, Status> {
    let post_id = database::delete_comment(db, user.id, id)
        .await
        .map_err(|_| Status::Unauthorized)?;
    Ok(Redirect::to(format!("/post/{}", post_id)))
}

/// Form struct for registering/logging in a user
#[derive(FromForm, Debug)]
pub struct UserForm {
    username: String,
    password: String,
}

/// Registers a user with the given username and password
/// args are automatically parsed from the request body
/// redirects to the index page if account creation was successful
/// redirects to the register page if the username is already taken
#[post("/register", data = "<user>")]
pub async fn register_user(
    db: Connection<database::MyDatabase>,
    user: Form<UserForm>,
    cookies: &CookieJar<'_>,
) -> Redirect {
    let user = match auth::User::new(db, user.username.clone(), user.password.clone()).await {
        Some(user) => user,
        None => return Redirect::to("/register"),
    };
    let claim = user.claim();
    cookies.add_private(Cookie::new("session", claim.to_string()));
    Redirect::to("/")
}

/// Logs in a user with the given username and password
/// args are automatically parsed from the request body
/// redirects to the index page if login was successful
/// redirects to the login page if the username or password is incorrect
#[post("/login", data = "<user>")]
pub async fn login_user(
    db: Connection<database::MyDatabase>,
    user: Form<UserForm>,
    cookies: &CookieJar<'_>,
) -> Redirect {
    let user = match auth::User::login(db, user.username.clone(), user.password.clone()).await {
        Some(user) => user,
        None => return Redirect::to("/login"),
    };
    let claim = user.claim();
    cookies.add_private(Cookie::new("session", claim.to_string()));
    Redirect::to("/")
}

/// Logs out the current user
/// args are automatically parsed from the request body
/// redirects to the index page
#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("session"));
    Redirect::to("/")
}

/// Creates a post from the given form
/// args are automatically parsed from the request body
/// redirects to the index page if the post was created successfully
/// only works if the user is logged in
#[post("/upload", data = "<post>")]
pub async fn create_post(
    user: auth::User,
    db: Connection<database::MyDatabase>,
    post: Form<files::Post<'_>>,
) -> Result<Redirect, Status> {
    files::create_post(db, post, user.id)
        .await
        .map_err(|_| Status::BadRequest)?;
    Ok(Redirect::to("/"))
}

/// form struct for creating a comment
#[derive(FromForm, Debug)]
pub struct Comment {
    body: String,
}

/// Creates a comment from the given form
/// args are automatically parsed from the request body
/// redirects to the post page if the comment was created successfully
#[post("/comment/<id>", data = "<comment>")]
pub async fn create_comment(
    id: i32,
    user: auth::User,
    db: Connection<database::MyDatabase>,
    comment: Form<Comment>,
) -> Result<Redirect, Status> {
    database::create_comment(
        db,
        database::Comment {
            id: None,
            owner: None,
            owner_id: user.id,
            post_id: id,
            body: comment.body.clone(),
        },
    )
    .await
    .map_err(|_| Status::BadRequest)?;
    Ok(Redirect::to(format!("/post/{id}")))
}
