/// Initializes `pretty_env_logger` wtih the `info` level. The environment variable `RUST_LOG`
/// redefines specified level.
pub fn init_with_info_level() {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
}
