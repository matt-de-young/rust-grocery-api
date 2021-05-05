#![allow(proc_macro_derive_resolution_fallback)]

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel;
use diesel::prelude::*;
use jsonwebtoken::TokenData;
use uuid::Uuid;

use crate::user::model::LoginCredentials;
use crate::user::model::LoginSession;
use crate::user::model::NewUser;
use crate::user::model::User;
use crate::user::model::UserToken;

use crate::schema::users;
use crate::schema::users::dsl::*;

pub fn create_user(new_user: NewUser, conn: &PgConnection) -> QueryResult<User> {
    let hashed_pwd = hash(&new_user.password, DEFAULT_COST).unwrap();
    let new_user = NewUser {
        password: hashed_pwd,
        ..new_user
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn login(login: LoginCredentials, conn: &PgConnection) -> Option<LoginSession> {
    let user_to_verify = users
        .filter(username.eq(&login.username))
        .or_filter(email.eq(&login.username))
        .get_result::<User>(conn)
        .unwrap();
    if !user_to_verify.password.is_empty()
        && verify(&login.password, &user_to_verify.password).unwrap()
    {
        let login_session_str = Uuid::new_v4().to_simple().to_string();
        update_login_session(&user_to_verify, &login_session_str, conn);
        Some(LoginSession {
            user_id: user_to_verify.id,
            login_session: login_session_str,
        })
    } else {
        None
    }
}

pub fn update_login_session(user: &User, update_login_session: &str, conn: &PgConnection) -> bool {
    diesel::update(user)
        .set(login_session.eq(update_login_session.to_string()))
        .execute(conn)
        .is_ok()
}

pub fn is_valid_login_session(user_token: &UserToken, conn: &PgConnection) -> bool {
    users
        .filter(id.eq(&user_token.user))
        .filter(login_session.eq(&user_token.login_session))
        .get_result::<User>(conn)
        .is_ok()
}

pub fn verify_token(token_data: &TokenData<UserToken>, conn: &PgConnection) -> bool {
    is_valid_login_session(&token_data.claims, conn)
}

pub fn get_user(user_id: i32, connection: &PgConnection) -> QueryResult<User> {
    users::table.find(user_id).get_result::<User>(connection)
}
