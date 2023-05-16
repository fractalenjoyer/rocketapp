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

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Comment {
    pub id: Option<i32>,
    pub owner: i32,
    pub post_id: i32,
    pub body: String,
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

pub async fn get_post_by_id(
    mut db: Connection<MyDatabase>,
    id: i32,
) -> Result<(Post, String, Vec<Comment>), sqlx::Error> {
    let post = sqlx::query("SELECT * FROM posts WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *db)
        .await?;

    let post = Post {
        id: Some(post.get("id")),
        owner: post.get("owner_id"),
        title: post.get("title"),
        body: post.get("body"),
        image: post.get("image"),
    };

    let poster = sqlx::query("SELECT username FROM users WHERE id = ?")
        .bind(post.owner)
        .fetch_one(&mut *db)
        .await?
        .get("username");

    let comments = match get_comments(db, id).await {
        Some(comments) => comments,
        None => Vec::new(),
    };

    Ok((post, poster, comments))
}

pub async fn create_comment(
    mut db: Connection<MyDatabase>,
    comment: Comment,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO comments (owner_id, post_id, body) VALUES (?, ?, ?)")
        .bind(comment.owner)
        .bind(comment.post_id)
        .bind(comment.body)
        .execute(&mut *db)
        .await?;
    Ok(())
}

pub async fn get_comments(mut db: Connection<MyDatabase>, post_id: i32) -> Option<Vec<Comment>> {
    let comments = sqlx::query("SELECT * FROM comments WHERE post_id = ? ORDER BY id DESC LIMIT 40")
        .bind(post_id)
        .fetch_all(&mut *db)
        .await
        .ok()?;

    let mut comments_vec = Vec::new();
    for comment in comments {
        comments_vec.push(Comment {
            id: Some(comment.get("id")),
            owner: comment.get("owner_id"),
            post_id: comment.get("post_id"),
            body: comment.get("body"),
        });
    }

    Some(comments_vec)
}

pub async fn get_posts(
    mut db: Connection<MyDatabase>,
) -> Result<(Vec<Post>, Vec<String>), sqlx::Error> {
    let posts = sqlx::query("SELECT * FROM posts ORDER BY id DESC LIMIT 20")
        .fetch_all(&mut *db)
        .await?;

    let mut posts_vec = Vec::new();
    let mut posters: Vec<String> = Vec::new();
    for post in posts {
        let owner_id: i32 = post.get("owner_id");
        posts_vec.push(Post {
            id: Some(post.get("id")),
            owner: owner_id,
            title: post.get("title"),
            body: post.get("body"),
            image: post.get("image"),
        });
        posters.push(
            sqlx::query("SELECT username FROM users WHERE id = ?")
                .bind(owner_id)
                .fetch_one(&mut *db)
                .await?
                .get("username"),
        )
    }

    Ok((posts_vec, posters))
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
use crate::files;

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
    let path: String = sqlx::query("SELECT image FROM posts WHERE id = ? AND owner_id = ?")
        .bind(post_id)
        .bind(owner_id)
        .fetch_one(&mut *db)
        .await
        .ok()?
        .get("image");
    sqlx::query("DELETE FROM posts WHERE id = ? AND owner_id = ?")
        .bind(post_id)
        .bind(owner_id)
        .execute(&mut *db)
        .await
        .ok()?;
    files::delete_file(path)?;
    Some(())
}
