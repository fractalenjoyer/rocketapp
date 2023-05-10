use rocket::form::Form;
use rocket::http::CookieJar;
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

#[derive(FromForm)]
pub struct UserForm {
    username: String,
    password: String,
}

#[post("/register", data = "<user>")]
pub async fn register_user(
    db: Connection<database::MyDatabase>,
    user: Form<UserForm>,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let user = auth::User::new(db, user.username.clone(), user.password.clone())
        .await?;
    Some(Template::render(
        "index",
        context! {
            title: "Hello!",
            style: "index.css",
        },
    ))
}


// #[get("/<id>")]
// pub async fn get_user(mut db: Connection<MyDatabase>, id: u32) -> Option<Template> {
//     let user = sqlx::query("SELECT * FROM users where id = ?")
//         .bind(id)
//         .fetch_one(&mut *db)
//         .await;
//     match user {
//         Ok(user) => {
//             let first_name: String = user.get("first_name");
//             let last_name: String = user.get("last_name");
//             Some(Template::render(
//                 "index",
//                 context! {
//                     name: format!("{first_name} {last_name}"),
//                     title: "Hello!",
//                     style: "index.css",
//                 },
//             ))
//         }
//         Err(_) => None,
//     }
// }

// #[get("/users")]
// pub async fn get_users(mut db: Connection<database::MyDatabase>) -> String {
//     sqlx::query("SELECT * FROM users")
//         .fetch_all(&mut *db)
//         .await
//         .unwrap()
//         .iter()
//         .map(|row| {
//             let first_name: String = row.get::<String, _>("first_name");
//             let last_name: String = row.get("last_name");
//             format!("{} {}", first_name, last_name)
//         })
//         .collect::<Vec<String>>()
//         .join("\n")
// }
