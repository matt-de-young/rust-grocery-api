#![allow(proc_macro_derive_resolution_fallback)]

use chrono::Utc;
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use jsonwebtoken::{Header, Validation};
use jsonwebtoken::{EncodingKey, DecodingKey};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use crate::connection::DbConn;
use crate::schema::users;
use crate::user::repository::verify_token;


static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Queryable, Identifiable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub login_session: Option<String>
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct LoginSession {
    pub username: String,
    pub login_session: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,    // issued at
    pub exp: i64,    // expiration
    pub user: String,
    pub login_session: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserToken, ()> {
        let conn = request.guard::<DbConn>().unwrap();
        if let Some(auth_header) = request.headers().get_one("Authorization") {    
            let auth_str = auth_header.to_string();
            if auth_str.starts_with("Bearer") {
                let token = auth_str[6..auth_str.len()].trim();
                if let Ok(token_data) = decode_token(token.to_string()) {
                    if verify_token(&token_data, &conn) {
                        return Outcome::Success(token_data.claims);
                    }
                } 
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

pub fn generate_token(login: LoginSession) -> ResponseToken {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = UserToken {
        iat: now,
        exp: now + ONE_WEEK,
        user: login.username,
        login_session: login.login_session,
    };
    ResponseToken {
        token: jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(include_bytes!("../secret.key"))).unwrap(),
        r#type: "Bearer".to_string()
    }
}

fn decode_token(token: String) -> Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(include_bytes!("../secret.key")), &Validation::default())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseToken {
    pub token: String,
    pub r#type: String
}
