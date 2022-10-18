use crate::db::{connection::establish_connection, resume::create_resume, resume::Resumes};

use actix_web::{web, HttpResponse};
use serde_derive::Serialize;
#[derive(Serialize)]
pub struct Test {
    pub message: String,
}
//joins
pub async fn write_resume(info: web::Json<Resumes>) -> HttpResponse {
    let connection = establish_connection();
    let cv_address = &info.cv_address.to_string();
    let cv_email = &info.cv_email.to_string();
    let cv_letter = &info.cv_letter.to_string();
    // let cv_tech = &info.cv_tech.to_string();
    let cv_edu = &&info.cv_edu.to_string();
    let cv_cert = &info.cv_cert.to_string();
    let cv_awards = &info.cv_awards.to_string();
    let cv_project = &info.cv_project.to_string();
    let user_resumekey = &info.cv_user_key;

    let _post = create_resume(
        &connection,
        cv_address,
        cv_email,
        cv_letter,
        cv_edu,
        cv_cert,
        cv_awards,
        cv_project,
        *user_resumekey,
    );
    HttpResponse::Ok().body("이력서 작성완료")
}
//get
// pub async fn get_data() -> HttpResponse {
//     let connection = establish_connection();
//     let _post = get_all(&connection);
//     HttpResponse::Ok().json(_post)
// }

//ㅇㅂㄹㅂ
