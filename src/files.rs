use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::response::Redirect;
use rocket_db_pools::Connection;

use uuid::Uuid;

use crate::database;

/// form struct for creating a post
#[derive(FromForm)]
pub struct Post<'r> {
    title: String,          // title of the post
    body: String,           // body of the post
    image: TempFile<'r>,    // image of the post
}


/// Creates a post with the given title, body and image
/// saves the image to the static/content folder with a random uuid as name
/// redirects to the upload page if the post was created successfully
/// only works if the user is logged in
/// args are automatically parsed from the request body
pub async fn create_post(
    db: Connection<database::MyDatabase>,
    mut post: Form<Post<'_>>,
    user_id: i32,
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
            body: post.body.clone(),
            image: img_path,
        },
    )
    .await
    .ok()?;
    Some(Redirect::to("/upload"))
}

/// Deletes a file from the static/content folder
/// name of the file is given as argument
pub fn delete_file(name: String) -> Option<()>{
    std::fs::remove_file(format!("static/content/{}", name)).ok()?;
    Some(())
}
