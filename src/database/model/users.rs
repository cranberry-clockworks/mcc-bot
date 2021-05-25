#[derive(sqlx::FromRow)]
pub struct User {
    id: i64,
    name: String,
    allow_post_vacancies: bool,
}
