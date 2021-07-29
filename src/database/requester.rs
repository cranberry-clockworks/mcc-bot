use crate::database::{DatabaseConnection, Vacancy};

pub struct Requester<'db> {
    db: &'db DatabaseConnection,
}

impl<'db> Requester<'db> {
    pub fn new(db: &'db DatabaseConnection) -> Requester<'db> {
        Self { db }
    }

    pub async fn insert_vacancy(&self, owner_id: i64, title: &str, description: &str) {
        let _ = sqlx::query(
            "INSERT INTO public.vacancies (owner_id, title, description) VALUES ($1, $2, $3)",
        )
        .bind(&owner_id)
        .bind(title)
        .bind(description)
        .execute(&self.db.connection)
        .await;
    }

    pub async fn select_vacancies(&self) -> Vec<Vacancy> {
        sqlx::query_as::<_, Vacancy>("SELECT * FROM public.vacancies")
            .fetch_all(&self.db.connection)
            .await
            .unwrap_or_else(|error| {
                log::error!("Failed to fetch vacancies. Error: {:#?}", error);
                return Vec::new();
            })
    }
}
