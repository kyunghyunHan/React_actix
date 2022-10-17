// use crate::db::user::get_all;
use crate::db::{
    connection::establish_connection,
    user::create_user,
    user::{Info, LoginUser, User},
};

use actix_identity::Identity;
use argonautica::Verifier;
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use dotenv::dotenv;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::Responder;
use actix_web::{web, Error, HttpResponse, ResponseError};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_derive::{Deserialize, Serialize};

// const JWT_SECRET: &str = "secret";
// const JWT_COOKIE_KEY: &str = "jwt";
#[derive(Debug, Serialize, Deserialize)]
pub struct Klaim {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Serialize)]
pub struct Test {
    pub message: String,
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
pub async fn process_login(data: web::Json<LoginUser>, id: Identity) -> HttpResponse {
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
                let klaim = Klaim {
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
                println!("{}", token);
                HttpResponse::Ok().body(token)
            } else {
                HttpResponse::Ok().body("s")
            }
        }
        Err(e) => {
            println!("{:?}", e);

            HttpResponse::Ok().body("s")
        }
    }
}

pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logged out.")
}
