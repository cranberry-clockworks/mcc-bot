pub enum ErrorCode {
    FailedConnectToDatabase = 1,
    BadEnvironment = 2,
    InvalidSettings = 3,
}

pub fn terminate(code: ErrorCode) -> ! {
    std::process::exit(code as i32);
}
