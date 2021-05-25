use crate::error::{terminate, ErrorCode};

use mccbot::database::DatabaseConnection;

pub async fn migrate(database: &DatabaseConnection) {
    sqlx::migrate!()
        .run(&database.connection)
        .await
        .unwrap_or_else(|e| {
            log::error!("Failed to perform migration! Error: {}", e);
            terminate(ErrorCode::FailedPerformMigration);
        });
}
