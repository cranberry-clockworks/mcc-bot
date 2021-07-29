#[derive(sqlx::FromRow)]
pub struct Vacancy {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub description: String,
}
