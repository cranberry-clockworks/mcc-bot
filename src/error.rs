pub enum ExitCode {
    Ok = 0,
    ConnectionFailure = 1,
    UserCreationFailure = 2,
    DatabaseCreationFailure = 3,
    MigrationFailure = 4,
}

pub fn terminate(code: ExitCode) -> ! {
    std::process::exit(code as i32);
}
