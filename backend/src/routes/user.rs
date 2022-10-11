use crate::db::user::get_all;
use crate::db::{connection::establish_connection, user::create_post};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    user_id: String,
    user_password: String,
    user_name: String,
    user_birth: String,
    user_address: String,
    user_email: String,
}

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
pub async fn get_data() -> HttpResponse {
    let connection = establish_connection();
    let _post = get_all(&connection);
    HttpResponse::Ok().json(_post)
}
