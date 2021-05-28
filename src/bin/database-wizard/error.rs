pub enum ErrorCode {
    InvalidSettings = 1,
    FailedEstablishConnectionWithDatabase = 2,
    FailedCreateUser = 3,
    FailedCreateDatabase = 4,
    FailedPerformMigration = 5,
}

pub fn terminate(code: ErrorCode) -> ! {
    std::process::exit(code as i32);
}
