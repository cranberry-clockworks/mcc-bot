use crate::database::DatabaseConnection;

pub struct Requester<'db> {
    db: &'db DatabaseConnection,
}

impl<'db> Requester<'db> {
    pub fn new(db: &'db DatabaseConnection) -> Requester<'db> {
        Self { db }
    }
}
