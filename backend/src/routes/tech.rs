use crate::db::{connection::establish_connection, tech::create_tech, tech::Techs};

use actix_web::{web, HttpResponse};

pub async fn write_tech(info: web::Json<Techs>) -> HttpResponse {
    let connection = establish_connection();
    let tc_list = &info.tc_list.to_string();
    let tc_user_key = &info.tc_user_key;
    let tc_resume_key = &info.tc_resume_key;

    let _post = create_tech(&connection, tc_list, *tc_user_key, *tc_resume_key);
    HttpResponse::Ok().body("기술 작성완료")
}
