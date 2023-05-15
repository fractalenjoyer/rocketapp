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
    pub id: Option<i32>,
    pub owner: i32,
    pub title: String,
    pub body: String,
    pub image: String,
}

pub async fn create_post(mut db: Connection<MyDatabase>, post: Post) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO posts (owner_id, title, body, image) VALUES (?, ?, ?, ?)")
        .bind(post.owner)
        .bind(post.title)
        .bind(post.body)
        .bind(post.image)
        .execute(&mut *db)
        .await?;
    Ok(())
}

pub async fn get_posts(
    mut db: Connection<MyDatabase>,
) -> Result<(Vec<Post>, Connection<MyDatabase>), sqlx::Error> {
    // this function is evil now

    let posts = sqlx::query("SELECT * FROM posts ORDER BY id DESC LIMIT 20")
        .fetch_all(&mut *db)
        .await?;

    let mut posts_vec = Vec::new();
    for post in posts {
        posts_vec.push(Post {
            id: Some(post.get("id")),
            owner: post.get("owner_id"),
            title: post.get("title"),
            body: post.get("body"),
            image: post.get("image"),
        });
    }

    Ok((posts_vec, db))
}

pub async fn get_users_by_posts(
    mut db: Connection<MyDatabase>,
    posts: &Vec<Post>,
) -> Result<Vec<User>, sqlx::Error> {
    let mut users = Vec::new();
    for post in posts {
        let user = sqlx::query("SELECT * FROM users WHERE id = ?")
            .bind(post.owner)
            .fetch_one(&mut *db)
            .await?;
        users.push(User {
            id: user.get("id"),
            username: user.get("username"),
            pw_hash: None,
        });
    }
    Ok(users)
}

pub async fn create_user(
    mut db: Connection<MyDatabase>,
    username: String,
    hash: String,
) -> Option<i32> {
    sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(username.clone())
        .bind(hash)
        .execute(&mut *db)
        .await
        .ok()?;
    println!("Created user {}", username);
    let id: u64 = sqlx::query("SELECT LAST_INSERT_ID() as id")
        .fetch_one(&mut *db)
        .await
        .ok()?
        .get("id");
    Some(id as i32)
}

pub async fn get_user_by_username(
    mut db: Connection<MyDatabase>,
    username: String,
) -> Option<User> {
    let user = sqlx::query("SELECT * FROM users where username = ?")
        .bind(username)
        .fetch_one(&mut *db)
        .await
        .ok()?;
    Some(User {
        id: user.get("id"),
        username: user.get("username"),
        pw_hash: user.get("password"),
    })
}

use crate::auth::User;

pub async fn get_user_by_id(mut db: Connection<MyDatabase>, sub: String) -> Option<User> {
    let user = sqlx::query("SELECT * FROM users where id = ?")
        .bind(sub.parse::<i64>().ok()?)
        .fetch_one(&mut *db)
        .await
        .ok()?;
    Some(User {
        id: user.get("id"),
        username: user.get("username"),
        pw_hash: user.get("password"),
    })
}

pub async fn delete_post(
    mut db: Connection<MyDatabase>,
    owner_id: i32,
    post_id: i32,
) -> Option<()> {
    sqlx::query("DELETE FROM posts WHERE id = ? AND owner_id = ?")
        .bind(post_id)
        .bind(owner_id)
        .execute(&mut *db)
        .await
        .ok()?;
    Some(())
}
