// use crate::db::user::get_all;
use crate::db::{
    connection::establish_connection,
    user::create_post,
    user::{Info, LoginUser, NewUser, User},
};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::Responder;
use actix_web::{web, HttpResponse};

// #[derive(Debug, Deserialize)]
// pub struct LoginUser {
//     pub user_id: String,
//     pub user_password: String,
// }
//joins
pub async fn write_data(info: web::Json<Info>) -> HttpResponse {
    let connection = establish_connection();
    let user_id = &info.user_id.to_string();
    let user_pw = &info.user_pw.to_string();
    let user_name = &info.user_name.to_string();
    let user_phone = &&info.user_phone.to_string();
    let _post = create_post(&connection, user_id, user_pw, user_name, user_phone);
    HttpResponse::Ok().body(info.user_id.to_string())
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
            if u.user_pw == data.user_pw {
                let session_token = String::from(u.user_id);
                id.remember(session_token);
                HttpResponse::Ok().body(format!("Logged in: {}", data.user_id))
            } else {
                HttpResponse::Ok().body("Password is incorrect.")
            }
        }
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::Ok().body("User doesn't exist.")
        }
    }
}
