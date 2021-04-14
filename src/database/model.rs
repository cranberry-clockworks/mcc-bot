#[derive(sqlx::FromRow)]
pub struct User {
    id: i64,
    name: String,
    allow_post_vacancies: bool,
}

#[derive(sqlx::FromRow)]
pub struct Vacancy {
    id: i64,
    owner_id: i64,
    title: String,
    description: String,
}
