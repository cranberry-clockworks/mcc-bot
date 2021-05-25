pub enum ErrorCode {
    FailedEstablishConnectionWithDatabase = 1,
    FailedCreateUser = 2,
    FailedCreateDatabase = 3,
    FailedPerformMigration = 4,
    FailedSetupEnvironment = 5,
}

pub fn terminate(code: ErrorCode) -> ! {
    std::process::exit(code as i32);
}
