use crate::database::DatabaseConnection;

pub struct Requester<'db> {
    db: &'db DatabaseConnection,
}

impl<'db> Requester<'db> {
    pub fn new(db: &'db DatabaseConnection) -> Requester<'db> {
        Self { db }
    }

    pub async fn insert_vacancy(&self, owner_id: i64, title: &str, description: &str) {
        let result = sqlx::query(
            "INSERT INTO public.vacancies (owner_id, title, description) VALUES ($1, $2, $3)",
        )
        .bind(&owner_id)
        .bind(title)
        .bind(description)
        .execute(&self.db.connection)
        .await;
        log::error!("{:#?}", result);
    }
}
