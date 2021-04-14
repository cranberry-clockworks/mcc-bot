use mccbot::database::DatabaseConnection;
use mccbot::error;

pub async fn run_migrations(connection: &DatabaseConnection) {
    sqlx::migrate!()
        .run(&connection.handle)
        .await
        .unwrap_or_else(|_| {
            log::error!("Failed to perform migration!");
            error::terminate(error::ExitCode::DatabaseCreationFailure);
        });
}
