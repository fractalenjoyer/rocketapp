use rocket::outcome::Outcome::{Failure, Success};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{self, Deserialize, Serialize};

use pwhash::bcrypt;
use rocket_db_pools::Connection;

use crate::database;

pub struct User {
    pub id: i32,
    pub username: String,
    pub pw_hash: Option<String>,
}

impl User {
    pub async fn new(
        db: Connection<database::MyDatabase>,
        username: String,
        password: String,
    ) -> Option<Self> {
        let hash = bcrypt::hash(password).ok()?;
        let id = database::create_user(db, username.clone(), hash.clone()).await?;
        Some(Self {
            id,
            username,
            pw_hash: None,
        })
    }

    pub async fn login(
        db: Connection<database::MyDatabase>,
        username: String,
        password: String,
    ) -> Option<Self> {
        let user = database::get_user_by_username(db, username.clone()).await?;
        if bcrypt::verify(password, &user.pw_hash.clone()?) {
            Some(user)
        } else {
            None
        }
    }

    pub fn claim(&self) -> Claim {
        Claim::new(
            self.id.to_string(),
            chrono::Utc::now().timestamp() as usize + 86400,
        )
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let claim: Option<Claim> = req
            .cookies()
            .get_private("session")
            .map(|cookie| serde::json::from_str::<Claim>(cookie.value()).unwrap());

        let claim = match claim {
            Some(claim) => claim,
            None => return Failure((rocket::http::Status::Unauthorized, ())),
        };

        if claim.exp < chrono::Utc::now().timestamp() as usize {
            return Failure((rocket::http::Status::Unauthorized, ()));
        }

        let db = req
            .guard::<Connection<database::MyDatabase>>()
            .await
            .unwrap();
        Success(database::get_user_by_id(db, claim.sub).await.unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Claim {
    sub: String,
    exp: usize,
}

impl Claim {
    fn new(sub: String, exp: usize) -> Self {
        Self { sub, exp }
    }
    pub fn to_string(&self) -> String {
        serde::json::to_string(self).unwrap()
    }
}
