use mccbot::database::DatabaseConnection;

pub struct QueryTool<'db> {
    database: &'db DatabaseConnection,
}

impl<'db> QueryTool<'db> {
    pub fn new(database: &'db DatabaseConnection) -> Self {
        Self { database }
    }

    pub async fn create_user(&self, name: &str, password: &str) -> Result<(), sqlx::Error> {
        assert!(name.chars().all(char::is_alphanumeric));
        assert!(password.chars().all(char::is_alphanumeric));

        let _ = sqlx::query(&format!(
            "CREATE USER {} WITH ENCRYPTED PASSWORD '{}'",
            &name, &password
        ))
        .execute(&self.database.connection)
        .await?;

        Ok(())
    }

    pub async fn create_database(&self, name: &str, owner: &str) -> Result<(), sqlx::Error> {
        assert!(name.chars().all(char::is_alphanumeric));
        assert!(owner.chars().all(char::is_alphanumeric));

        let _ = sqlx::query(&format!(
            "CREATE DATABASE \"{}\" WITH OWNER = {} ENCODING = 'UTF-8'",
            name, owner
        ))
        .execute(&self.database.connection)
        .await?;

        Ok(())
    }
}
