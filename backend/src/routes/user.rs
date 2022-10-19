use crate::db::{
    connection::establish_connection,
    user::create_user,
    user::{GetUser, Info, LoginUser, User},
};
use actix_identity::Identity;
use argonautica::Verifier;
// use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use dotenv::dotenv;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub nbf: i64,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tokens {
    pub user_id: String,
    pub user_name: String,
    pub message: String,
    pub accesstoken: String,
    pub refreshtoken: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokens {
    pub accesstoken: String,
    pub refreshtoken: String,
    pub user_id: String,
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
                let acc_exp = iat + Duration::days(7);
                let ref_exp = iat + Duration::hours(1);
                let mut claim = Claims {
                    // iss: u.user_id,
                    sub: "Quest".to_string(),
                    nbf: iat.timestamp_nanos(),
                    exp: acc_exp.timestamp_nanos(),
                };
                let access_token = encode(
                    &Header::default(),
                    &claim,
                    &EncodingKey::from_secret(secret.as_bytes()),
                )
                .unwrap();
                claim.exp = ref_exp.timestamp_nanos();
                let refresh_token = encode(
                    &Header::default(),
                    &claim,
                    &EncodingKey::from_secret(secret.as_bytes()),
                )
                .unwrap();
                web::Json(Tokens {
                    user_id: u.user_id,
                    message: "로그인성공".to_string(),
                    user_name: u.user_name,
                    accesstoken: access_token,
                    refreshtoken: refresh_token,
                })
            } else {
                web::Json(Tokens {
                    user_id: "-".to_string(),
                    user_name: "-".to_string(),
                    message: "비밀번호틀림".to_string(),
                    accesstoken: "-".to_string(),
                    refreshtoken: "".to_string(),
                })
            }
        }
        Err(e) => {
            println!("{:?}", e);

            web::Json(Tokens {
                user_id: "-".to_string(),
                user_name: "-".to_string(),
                message: "정보틀림".to_string(),
                accesstoken: "-".to_string(),
                refreshtoken: "".to_string(),
            })
        }
    }
}

pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logged out.")
}

//체크 토큰
pub async fn check_token(data: web::Json<GetTokens>) -> impl Responder {
    dotenv().ok();
    use crate::db::schema::users::dsl::{user_id, users};
    let connection = establish_connection();

    let secret = std::env::var("APP_SECRET").expect("SECRET_KEY must be set");
    let ref_token = decode::<Claims>(
        &data.refreshtoken,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .unwrap();

    let acc_token = decode::<Claims>(
        &data.accesstoken,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .unwrap();
    let now_time = Utc::now().timestamp_nanos();
    // .unwrap();

    let user = users
        .filter(user_id.eq(&data.user_id))
        .first::<User>(&connection);

    match user {
        Ok(u) => {
            if ref_token.claims.exp > now_time {
                if acc_token.claims.exp > now_time {
                    web::Json(Tokens {
                        user_id: u.user_id,
                        user_name: u.user_name,
                        message: "accTokIsLogin".to_string(),
                        accesstoken: "-".to_string(),
                        refreshtoken: "".to_string(),
                    })
                } else {
                    let secret = std::env::var("APP_SECRET").expect("SECRET_KEY must be set");
                    let iat = Utc::now();
                    let acc_exp = iat + Duration::days(7);
                    let claim = Claims {
                        // iss: u.user_id,
                        sub: "Quest".to_string(),
                        nbf: iat.timestamp_nanos(),
                        exp: acc_exp.timestamp_nanos(),
                    };
                    let access_token = encode(
                        &Header::default(),
                        &claim,
                        &EncodingKey::from_secret(secret.as_bytes()),
                    )
                    .unwrap();
                    web::Json(Tokens {
                        user_id: "-".to_string(),
                        user_name: "-".to_string(),
                        message: "accTokCreateLogin".to_string(),
                        accesstoken: access_token,
                        refreshtoken: "".to_string(),
                    })
                }
            } else {
                web::Json(Tokens {
                    user_id: "-".to_string(),
                    user_name: "-".to_string(),
                    message: "tokenEnd".to_string(),
                    accesstoken: "-".to_string(),
                    refreshtoken: "".to_string(),
                })
            }
        }
        Err(e) => web::Json(Tokens {
            user_id: "-".to_string(),
            user_name: "-".to_string(),
            message: "userUndifind".to_string(),
            accesstoken: "-".to_string(),
            refreshtoken: "".to_string(),
        }),
    }
}

//아이디 중복확인
pub async fn check_id(data: web::Json<GetUser>) -> impl Responder {
    use crate::db::schema::users::dsl::{user_id, users};
    let connection = establish_connection();

    let user = users
        .filter(user_id.eq(&data.user_id))
        .first::<User>(&connection);
    match user {
        Ok(u) => {
            if u.user_id == data.user_id {
                HttpResponse::Ok().body("아이디가 있습니다")
            } else {
                HttpResponse::Ok().body("사용 가능한 아이디 입니다.")
            }
        }
        Err(e) => HttpResponse::Ok().body("사용 가능한 아이디 입니다."),
    }
}
