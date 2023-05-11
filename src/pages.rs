use rocket::form::Form;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::auth;
use crate::database;
use crate::files;

#[get("/")]
pub async fn index(db: Connection<database::MyDatabase>) -> Option<Template> {
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
pub fn create() -> Template {
    Template::render(
        "upload",
        context! {
            title: "Upload",
            style: "upload.css"
        },
    )
}

#[post("/new", data = "<post>")]
pub async fn create_post(
    user: auth::User,
    db: Connection<database::MyDatabase>,
    post: Form<files::Post<'_>>,
) -> Option<Template> {
    files::create_post(db, post, user.id).await?;
    Some(Template::render(
        "index",
        context! {
            title: "Hello!",
            style: "index.css",
        },
    ))
}

#[get("/register")]
pub fn register() -> Template {
    Template::render(
        "register",
        context! {
            title: "Register",
            style: "register.css"
        },
    )
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
