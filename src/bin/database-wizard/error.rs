pub enum ErrorCode {
    ConnectionFailure = 1,
    UserCreationFailure = 2,
    DatabaseCreationFailure = 3,
    MigrationFailure = 4,
}

pub fn terminate(code: ErrorCode) -> ! {
    std::process::exit(code as i32);
}
