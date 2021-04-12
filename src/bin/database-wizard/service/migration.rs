pub async fn run_migrations(connection: &sqlx::Pool<sqlx::Postgres>) {
    sqlx::migrate!()
        .run(connection)
        .await
        .expect("Failed to migrate!");
}
