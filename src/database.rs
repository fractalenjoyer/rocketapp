/// This file contains all of the sites database operations
/// 
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::Connection;
use rocket_db_pools::{sqlx, Database};

use rocket::serde::Serialize;

use crate::auth::User;
use crate::files;

/// Derive the Database trait for our database struct
/// This allows attaching the database to the rocket app
#[derive(Database)]
#[database("mysql")]
pub struct MyDatabase(sqlx::MySqlPool);

/// Structs for the database tables
/// id is optional because it is generated by the database
#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: Option<i32>, // the id of the post
    pub owner: i32,      // the id of the user who created this post aka User.id
    pub title: String,   // the title of the post
    pub body: String,    // the body of the post
    pub image: String,   // the filename of the image
}

/// Structs for the database tables
/// id is optional because it is generated by the database
#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Comment {
    pub id: Option<i32>,       // the id of the comment
    pub owner: Option<String>, // the username of the user who created this comment
    pub owner_id: i32,         // the id of the user who created this comment aka User.id
    pub post_id: i32,          // the id of the post this comment belongs to aka Post.id
    pub body: String,          // the body of the comment
}

/// creates a post with the given data
/// returns an error if the post could not be created
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

/// creates a comment with the given data
/// returns an error if the comment could not be created
pub async fn create_comment(
    mut db: Connection<MyDatabase>,
    comment: Comment,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO comments (owner_id, post_id, body) VALUES (?, ?, ?)")
        .bind(comment.owner_id)
        .bind(comment.post_id)
        .bind(comment.body)
        .execute(&mut *db)
        .await?;
    Ok(())
}

/// creates a user with the given username and password hash
/// returns the id of the created user or None if the user could not be created
/// returns None if the username is already taken
pub async fn create_user(
    mut db: Connection<MyDatabase>,
    username: String,
    hash: String,
) -> Result<i32, Box<dyn std::error::Error>> {
    match sqlx::query("SELECT * FROM users WHERE username = ?")
        .bind(username.clone())
        .fetch_optional(&mut *db)
        .await?
    {
        Some(_) => return Err("Username already taken".into()),
        None => {}
    }
    sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(username.clone())
        .bind(hash)
        .execute(&mut *db)
        .await?;
    println!("Created user {}", username);
    let id: u64 = sqlx::query("SELECT LAST_INSERT_ID() as id")
        .fetch_one(&mut *db)
        .await?
        .get("id");
    Ok(id as i32)
}

/// returns a post along with the username of the owner and all comments
/// returns an error if the post does not exist
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
        Ok(comments) => comments,
        Err(_) => Vec::new(),
    };

    Ok((post, poster, comments))
}

/// get comments for a post
/// returns a vector of comments for the given post id
/// returns None if the database query fails
pub async fn get_comments(mut db: Connection<MyDatabase>, post_id: i32) -> Result<Vec<Comment>, sqlx::Error> {
    let comments =
        sqlx::query("SELECT * FROM comments WHERE post_id = ? ORDER BY id DESC LIMIT 40")
            .bind(post_id)
            .fetch_all(&mut *db)
            .await?;

    let mut comments_vec = Vec::new();
    for comment in comments {
        let owner_id = comment.get("owner_id");
        let owner = sqlx::query("SELECT username FROM users WHERE id = ?")
            .bind(owner_id)
            .fetch_one(&mut *db)
            .await?
            .get("username");
        comments_vec.push(Comment {
            id: Some(comment.get("id")),
            owner,
            owner_id,
            post_id: comment.get("post_id"),
            body: comment.get("body"),
        });
    }

    Ok(comments_vec)
}

/// Get the 20 most recent posts along with the username of the owner
/// returns a vector of posts and a vector of usernames
/// returns an error if the database query fails
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

/// Get the 20 most recent posts along with the username of the owner given a username
/// returns a vector of posts and a vector of usernames
/// returns an error if the database query fails
pub async fn get_posts_by_user(
    mut db: Connection<MyDatabase>,
    username: String,
) -> Result<(Vec<Post>, Vec<String>), sqlx::Error> {
    let posts = sqlx::query("SELECT * FROM posts WHERE owner_id = (SELECT id FROM users WHERE username = ?) ORDER BY id DESC LIMIT 20")
        .bind(username)
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

/// Get userdetails for a given username
/// returns a user struct
/// returns an Error if the database query fails or the user does not exist
pub async fn get_user_by_username(
    mut db: Connection<MyDatabase>,
    username: String,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query("SELECT * FROM users where username = ?")
        .bind(username)
        .fetch_one(&mut *db)
        .await?;
    Ok(User {
        id: user.get("id"),
        username: user.get("username"),
        pw_hash: user.get("password"),
    })
}

/// Get userdetails for a given id
/// returns a user struct
/// returns an error if the database query fails or the user does not exist
/// returns an error if the id cannot be parsed to an i64
pub async fn get_user_by_id(
    mut db: Connection<MyDatabase>,
    sub: String,
) -> Result<User, Box<dyn std::error::Error>> {
    let user = sqlx::query("SELECT * FROM users where id = ?")
        .bind(sub.parse::<i64>()?)
        .fetch_one(&mut *db)
        .await?;
    Ok(User {
        id: user.get("id"),
        username: user.get("username"),
        pw_hash: user.get("password"),
    })
}

/// Delete a post given the owner id and post id
/// returns None if the database query fails
/// returns None if the file cannot be deleted
/// returns Some(()) if the post is deleted
pub async fn delete_post(
    mut db: Connection<MyDatabase>,
    owner_id: i32,
    post_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let path: String = sqlx::query("SELECT image FROM posts WHERE id = ? AND owner_id = ?")
        .bind(post_id)
        .bind(owner_id)
        .fetch_one(&mut *db)
        .await?
        .get("image");
    sqlx::query("DELETE FROM posts WHERE id = ? AND owner_id = ?")
        .bind(post_id)
        .bind(owner_id)
        .execute(&mut *db)
        .await?;
    files::delete_file(path)?;
    Ok(())
}

/// Delete a comment given the owner id and comment id
/// returns None if the database query fails
/// returns Some(()) if the comment is deleted
pub async fn delete_comment(
    mut db: Connection<MyDatabase>,
    owner_id: i32,
    comment_id: i32,
) -> Result<i32, sqlx::Error> {
    let post_id = sqlx::query("SELECT post_id FROM comments WHERE id = ? AND owner_id = ?")
        .bind(comment_id)
        .bind(owner_id)
        .fetch_one(&mut *db)
        .await?
        .get("post_id");
    sqlx::query("DELETE FROM comments WHERE id = ? AND owner_id = ?")
        .bind(comment_id)
        .bind(owner_id)
        .execute(&mut *db)
        .await?;

    Ok(post_id)
}
