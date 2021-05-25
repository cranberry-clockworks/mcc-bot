use crate::database::DatabaseConnection;

static TELEGRAM_TOKEN_ID: &str = "telegram";

#[derive(sqlx::FromRow)]
pub struct Token {
    pub id: String,
    pub token: String,
}

impl Token {
    pub async fn insert(&self, database: &DatabaseConnection) -> Result<(), sqlx::Error> {
        let _ = sqlx::query("INSERT INTO public.tokens VALUES ($1, $2)")
            .bind(&self.id)
            .bind(&self.token)
            .execute(&database.connection)
            .await?;

        Ok(())
    }

    pub async fn insert_telegram_token(
        token: &str,
        database: &DatabaseConnection,
    ) -> Result<(), sqlx::Error> {
        Self {
            id: TELEGRAM_TOKEN_ID.to_string(),
            token: token.to_string(),
        }
        .insert(database)
        .await?;

        Ok(())
    }
}
