use rocket::form::Form;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::response::content::RawJson;
use rocket::response::Redirect;
use rocket_db_pools::Connection;

use crate::auth;
use crate::database;
use crate::files;

#[get("/whoami")]
pub fn whoami(user: auth::User) -> String {
    format!("Hello, {}!", user.username)
}

#[get("/userdetails/<id>")]
pub async fn userdetails(
    id: String,
    db: Connection<database::MyDatabase>,
) -> Option<RawJson<String>> {
    let user = database::get_user_by_id(db, id.clone()).await?;

    Some(RawJson(format!(
        r#"{{ 
            "username": {:?}
         }}"#,
        user.username
    )))
}

#[get("/deletepost/<id>")]
pub async fn deletepost(
    id: i32,
    user: auth::User,
    db: Connection<database::MyDatabase>,
) -> Option<Redirect> {
    database::delete_post(db, user.id, id).await?;
    Some(Redirect::to("/"))
}

#[derive(FromForm, Debug)]
pub struct UserForm {
    username: String,
    password: String,
}

#[post("/register", data = "<user>")]
pub async fn register_user(
    db: Connection<database::MyDatabase>,
    user: Form<UserForm>,
    cookies: &CookieJar<'_>,
) -> Option<Redirect> {
    let user = auth::User::new(db, user.username.clone(), user.password.clone()).await?;
    let claim = user.claim();
    cookies.add_private(Cookie::new("session", claim.to_string()));
    Some(Redirect::to("/"))
}

#[post("/login", data = "<user>")]
pub async fn login_user(
    db: Connection<database::MyDatabase>,
    user: Form<UserForm>,
    cookies: &CookieJar<'_>,
) -> Option<Redirect> {
    let user = match auth::User::login(db, user.username.clone(), user.password.clone()).await {
        Some(user) => user,
        None => return Some(Redirect::to("/login")),
    };
    let claim = user.claim();
    cookies.add_private(Cookie::new("session", claim.to_string()));
    Some(Redirect::to("/"))
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("session"));
    Redirect::to("/")
}

#[post("/upload", data = "<post>")]
pub async fn create_post(
    user: auth::User,
    db: Connection<database::MyDatabase>,
    post: Form<files::Post<'_>>,
) -> Option<Redirect> {
    files::create_post(db, post, user.id).await?;
    Some(Redirect::to("/"))
}

#[derive(FromForm, Debug)]
pub struct Comment {
    body: String,
}

#[post("/comment/<id>", data = "<comment>")]
pub async fn comment(
    id: i32,
    user: auth::User,
    db: Connection<database::MyDatabase>,
    comment: Form<Comment>,
) -> Option<Redirect> {
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
    .ok()?;
    Some(Redirect::to(format!("/post/{id}")))
}
