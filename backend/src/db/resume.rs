//이력서
#[derive(Debug, Queryable)]
pub struct Resume {
    pub id: i32,
    pub Cv_address: String,
    pub Cv_email: String,
    pub Cv_letter: String,
    pub Cv_tech: String,
    pub Cv_Edu: String,
    pub Cv_Cert: String,
    pub CV_Awards: String,
    pub created_at: chrono::NaiveDateTime,
}
