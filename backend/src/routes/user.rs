// use crate::db::user::get_all;
use crate::db::{
    connection::establish_connection,
    user::create_user,
    // user::find_user_by_uid,
    user::{GetUser, Info, LoginUser, User},
};
use actix_identity::Identity;
use argonautica::Verifier;
// use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use std::collections::HashMap;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::Responder;
use actix_web::{web, Error, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_derive::{Deserialize, Serialize};

const JWT_SECRET: &str = "secret";
const JWT_COOKIE_KEY: &str = "jwt";
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Serialize)]
pub struct Test {
    pub message: String,
    pub token: String,
}

pub async fn write_data(info: web::Json<Info>) -> HttpResponse {
    let connection = establish_connection();
    let user_id = &info.user_id.to_string();
    let user_pw = &info.user_pw.to_string();
    let user_name = &info.user_name.to_string();
    let user_phone = &&info.user_phone.to_string();
    let _post = create_user(&connection, user_id, user_pw, user_name, user_phone);
    HttpResponse::Ok().body("가입완료")
}
//get
// pub async fn get_data() -> HttpResponse {
//     let connection = establish_connection();
//     let _post = get_all(&connection);
//     HttpResponse::Ok().json(_post)
// }
pub async fn process_login(data: web::Json<LoginUser>, id: Identity) -> impl Responder {
    use crate::db::schema::users::dsl::{user_id, users};

    let connection = establish_connection();
    let user = users
        .filter(user_id.eq(&data.user_id))
        .first::<User>(&connection);
    match user {
        Ok(u) => {
            dotenv().ok();
            let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
            let valid = Verifier::default()
                .with_hash(u.user_pw)
                .with_password(data.user_pw.clone())
                .with_secret_key(secret)
                .verify()
                .unwrap();
            if valid {
                let secret = std::env::var("APP_SECRET").expect("SECRET_KEY must be set");
                let iat = Utc::now();
                let exp = iat + Duration::days(7);
                let klaim = Claims {
                    sub: u.user_id,
                    iat: iat.timestamp_nanos(),
                    exp: exp.timestamp_nanos(),
                };
                let token = encode(
                    &Header::default(),
                    &klaim,
                    &EncodingKey::from_secret(secret.as_bytes()),
                )
                .unwrap();
                web::Json(Test {
                    message: "로그인성공".to_string(),
                    token: token,
                })
                // HttpResponse::Ok().body(token)
            } else {
                web::Json(Test {
                    message: "유저가아님".to_string(),
                    token: "-".to_string(),
                })
            }
        }
        Err(e) => {
            println!("{:?}", e);

            web::Json(Test {
                message: "로그인성공".to_string(),
                token: "-".to_string(),
            })
        }
    }
}

pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logged out.")
}
fn dec_jwt(secret: &String, jwt: &String) -> Option<String> {
    let validation = jsonwebtoken::Validation::default();
    match jsonwebtoken::decode::<Claims>(
        &jwt,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(c) => Option::Some(c.claims.sub),

        _ => Option::None,
    }
}
fn get_cookie_map(req: actix_web::HttpRequest) -> HashMap<String, String> {
    match get_cookie_string(req) {
        Some(cookie_str) => {
            let cookies: Vec<&str> = cookie_str.split(";").collect();
            cookies
                .iter()
                .fold(HashMap::<String, String>::new(), |mut acc, cur| {
                    let entry: Vec<&str> = cur.split("=").collect();
                    acc.insert(entry[0].to_string(), entry[1].to_string());
                    acc
                })
        }
        None => HashMap::new(),
    }
}
fn get_cookie_string(req: actix_web::HttpRequest) -> Option<String> {
    let cookie_header = req.headers().get("cookie");
    if let Some(v) = cookie_header {
        let cookie_string = v.to_str().unwrap();
        return Some(String::from(cookie_string));
    }
    return None;
}
/// Finds user by UID.

pub async fn get_user(
    req: actix_web::HttpRequest,
    // data: web::Json<GetUser>,
    data: web::Path<GetUser>,
) -> Result<HttpResponse, Error> {
    let data = data.into_inner();
    use crate::db::schema::users::dsl::{user_id, users};

    let conn = establish_connection();
    let jwt = get_cookie_map(req)
        .get(JWT_COOKIE_KEY)
        .cloned()
        .unwrap_or_default();
    match dec_jwt(&JWT_SECRET.to_string(), &jwt) {
        Some(_) => {
            let userss = users.filter(user_id.eq(&data.user_id)).first::<User>(&conn);
            match userss {
                Ok(u) => Ok(HttpResponse::Ok().json(u)),
                _ => Ok(HttpResponse::NonAuthoritativeInformation().body("user not found.")),
            }
        }
        _ => Ok(HttpResponse::NonAuthoritativeInformation().body("invalid token.")),
    }
}
