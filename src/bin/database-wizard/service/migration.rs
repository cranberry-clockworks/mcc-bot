use mccbot::error;

pub async fn run_migrations(connection: &sqlx::Pool<sqlx::Postgres>) {
    sqlx::migrate!().run(connection).await.unwrap_or_else(|_| {
        log::error!("Failed to perform migration!");
        error::terminate(error::ExitCode::DatabaseCreationFailure);
    });
}
