use rocket_db_pools::sqlx::Row;
use rocket_db_pools::Connection;
use rocket_db_pools::{sqlx, Database};
use rocket_dyn_templates::{context, Template};

use rocket::serde::Serialize;

#[derive(Database)]
#[database("mysql")]
pub struct MyDatabase(sqlx::MySqlPool);
#[get("/<id>")]
pub async fn get_user(mut db: Connection<MyDatabase>, id: u32) -> Option<Template> {
    let user = sqlx::query("SELECT * FROM users where id = ?")
        .bind(id)
        .fetch_one(&mut *db)
        .await;
    match user {
        Ok(user) => {
            let first_name: String = user.get("first_name");
            let last_name: String = user.get("last_name");
            Some(Template::render("index", context! {
                name: format!("{} {}", first_name, last_name),
                title: "Hello!",
                style: "index.css",
            }))
        }
        Err(_) => None,
    }
}
#[get("/users")]
pub async fn get_users(mut db: Connection<MyDatabase>) -> String {
    sqlx::query("SELECT * FROM users")
        .fetch_all(&mut *db)
        .await
        .unwrap()
        .iter()
        .map(|row| {
            let first_name: String = row.get::<String, _>("first_name");
            let last_name: String = row.get("last_name");
            format!("{} {}", first_name, last_name)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub body: String,
    pub image: String,
}

pub async fn create_post(mut db: Connection<MyDatabase>, post: Post) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO posts (title, body, image) VALUES (?, ?, ?)")
        .bind(post.title)
        .bind(post.body)
        .bind(post.image)
        .execute(&mut *db)
        .await?;
    Ok(())
}

pub async fn get_posts(mut db: Connection<MyDatabase>) -> Result<Vec<Post>, sqlx::Error> {
    let posts = sqlx::query("SELECT * FROM posts")
        .fetch_all(&mut *db)
        .await?;
    let mut posts_vec = Vec::new();
    for post in posts {
        posts_vec.push(Post {
            id: Some(post.get("id")),
            title: post.get("title"),
            body: post.get("body"),
            image: post.get("image"),
        });
    }
    Ok(posts_vec)
}