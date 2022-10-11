// use crate::db::user::get_all;
use crate::db::{
    connection::establish_connection,
    user::create_post,
    user::{Info, LoginUser, NewUser, User},
};
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
    let user_password = &info.user_password.to_string();
    let user_name = &info.user_name.to_string();
    let user_birth = &info.user_birth.to_string();
    let user_address = &info.user_address.to_string();
    let user_email = &info.user_email.to_string();
    let _post = create_post(
        &connection,
        user_id,
        user_password,
        user_name,
        user_birth,
        user_address,
        user_email,
    );
    HttpResponse::Ok().body(info.user_id.to_string())
}
//get
// pub async fn get_data() -> HttpResponse {
//     let connection = establish_connection();
//     let _post = get_all(&connection);
//     HttpResponse::Ok().json(_post)
// }

pub async fn process_login(data: web::Json<LoginUser>) -> impl Responder {
    use crate::db::schema::users::dsl::{user_id, users};

    let connection = establish_connection();
    let user = users
        .filter(user_id.eq(&data.user_id))
        .first::<User>(&connection);

    match user {
        Ok(u) => {
            if u.user_password == data.user_password {
                println!("{:?}", data);
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
