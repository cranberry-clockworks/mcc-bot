#[derive(sqlx::FromRow)]
pub struct Vacancy {
    id: i64,
    owner_id: i64,
    title: String,
    description: String,
}
