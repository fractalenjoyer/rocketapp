use rocket::outcome::Outcome::{Failure, Success};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{self, Deserialize, Serialize};

use pwhash::bcrypt;
use rocket_db_pools::Connection;

use crate::database;

/// struct for storing a user instance
/// hash is optional since it is only used for password verification
pub struct User {
    pub id: i32,
    pub username: String,
    pub pw_hash: Option<String>,
}

/// implementation of User
impl User {
    /// create a new user
    /// hashes the password and stores the hash in the database
    /// returns None if the username is already taken
    /// returns Self if the user was created successfully
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

    /// login a user
    /// returns None if the username or password is incorrect
    /// returns Self if the user was logged in successfully
    pub async fn login(
        db: Connection<database::MyDatabase>,
        username: String,
        password: String,
    ) -> Option<Self> {
        let user = database::get_user_by_username(db, username.clone()).await.ok()?;
        if bcrypt::verify(password, &user.pw_hash.clone()?) {
            Some(user)
        } else {
            None
        }
    }

    /// create a claim for the user
    /// expires after 24 hours
    pub fn claim(&self) -> Claim {
        Claim::new(
            self.id.to_string(),
            chrono::Utc::now().timestamp() as usize + 86400,
        )
    }
}


/// trait implementation for User
/// allows for the use of User as a request guard
/// returns the user if the user is logged in
/// returns an error if the user is not logged in
/// the user is logged in if the session cookie is present and the session has not expired
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

/// struct for storing a claim
/// sub is the user id
/// exp is the expiration time
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Claim {
    sub: String,
    exp: usize,
}

/// implementation of Claim
impl Claim {
    /// create a new claim
    fn new(sub: String, exp: usize) -> Self {
        Self { sub, exp }
    }

    /// serialize the claim to a json string
    pub fn to_string(&self) -> String {
        serde::json::to_string(self).unwrap()
    }
}
