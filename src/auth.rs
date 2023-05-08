use rocket::request::{FromRequest, Request, Outcome};
use rocket::outcome::Outcome::{Success, Failure};
use rocket_db_pools::Connection;

use crate::database;

pub struct User {
    username: String,
}

pub struct SuperUser {
    username: String,
}

trait UserTrait {
    fn get_username(&self) -> String;
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let session = req.cookies().get_private("session").map(|crumb| crumb.value().to_string());
        let db = req.guard::<Connection<database::MyDatabase>>().await.unwrap();
        match session {
            Some(session) => {
                let user = database::get_user_by_session(db, session).await.unwrap();
                Success(User {
                    username: user,
                })
            }
            None => Failure((rocket::http::Status::Unauthorized, ())),
        }
        
    }
}