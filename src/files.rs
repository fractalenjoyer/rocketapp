use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::response::Redirect;
use rocket_db_pools::Connection;

use uuid::Uuid;

use crate::database;

#[derive(FromForm)]
pub struct Post<'r> {
    title: String,
    image: TempFile<'r>,
}

pub async fn create_post(
    db: Connection<database::MyDatabase>,
    mut post: Form<Post<'_>>,
    user_id: i64
) -> Option<Redirect> {
    println!("Title: {}", post.title);
    let img_path = format!("{}.png", Uuid::new_v4());
    post.image
        .persist_to(format!("static/content/{img_path}",))
        .await
        .ok()?;
    database::create_post(
        db,
        database::Post {
            id: None,
            owner: user_id,
            title: post.title.clone(),
            body: String::new(),
            image: img_path,
        },
    )
    .await
    .ok()?;
    Some(Redirect::to("/upload"))
}
