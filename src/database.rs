use rocket_db_pools::sqlx::Row;
use rocket_db_pools::Connection;
use rocket_db_pools::{sqlx, Database};

use rocket::serde::Serialize;

#[derive(Database)]
#[database("mysql")]
pub struct MyDatabase(sqlx::MySqlPool);


#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: Option<i64>,
    pub owner: i64,
    pub title: String,
    pub body: String,
    pub image: String,
}

pub async fn create_post(mut db: Connection<MyDatabase>, post: Post) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO posts (owner, title, body, image) VALUES (?, ?, ?)")
        .bind(post.owner)
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
            owner: post.get("owner"),
            title: post.get("title"),
            body: post.get("body"),
            image: post.get("image"),
        });
    }
    Ok(posts_vec)
}

pub async fn create_user(
    mut db: Connection<MyDatabase>,
    username: String,
    hash: String,
) -> Option<i64> {
    sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(username)
        .bind(hash)
        .execute(&mut *db)
        .await
        .ok()?;
    let id: i64 = sqlx::query("SELECT LAST_INSERT_ID() as id")
        .fetch_one(&mut *db)
        .await
        .ok()?
        .get("id");
    Some(id)
}

// pub async fn login_user(
//     mut db: Connection<MyDatabase>,
//     username: String,
//     password: String,
// ) -> Option<()> {
//     let user = sqlx::query("SELECT * FROM users where username = ?")
//         .bind(username)
//         .fetch_one(&mut *db)
//         .await
//         .ok()?;
//     let hash: String = user.get("password");
// }

use crate::auth::User;

pub async fn get_user (
    mut db: Connection<MyDatabase>,
    sub: String
) -> Option<User> {
    let user = sqlx::query("SELECT * FROM users where id = ?")
        .bind(sub.parse::<i64>().ok()?)
        .fetch_one(&mut *db)
        .await
        .ok()?;
    Some(User {
        id: user.get("id"),
        username: user.get("username"),
    })
}