//이력서
#[derive(Debug, Queryable)]
pub struct Resume {
    pub id: i32,
    pub Cv_address: String,
    pub Cv_email: String,
    pub Cv_letter: String,
    pub Cv_tech: String,
    pub created_at: chrono::NaiveDateTime,
}
