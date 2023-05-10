use rocket::outcome::Outcome::{Failure, Success};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{self, Deserialize, Serialize};

use pwhash::bcrypt;
use rocket_db_pools::Connection;

use crate::database;

pub struct User {
    pub id: i64,
    pub username: String,
}

impl User {
    pub async fn new (
        db: Connection<database::MyDatabase>,
        username: String,
        password: String,
    ) -> Option<Self> {
        let hash = bcrypt::hash(password).ok()?;
        let id = database::create_user(db, username.clone(), hash).await?;
        Some(Self { id, username })
    }

    pub fn claim(&self) -> Claim {
        Claim::new(self.id.to_string(), chrono::Utc::now().timestamp() as usize + 86400)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let claim = if let Some(session) = req.cookies().get_private("session") {
            serde::json::from_str::<Claim>(session.value()).unwrap()
        } else {
            return Failure((rocket::http::Status::Unauthorized, ()));
        };
        let db = req
            .guard::<Connection<database::MyDatabase>>()
            .await
            .unwrap();
        Success(database::get_user(db, claim.sub).await.unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Claim {
    sub: String,
    exp: usize,
}

impl Claim {
    fn new(sub: String, exp: usize) -> Self {
        Self { sub, exp }
    }
}
