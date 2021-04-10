use clap::Clap;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Options {
    #[clap(subcommand)]
    pub command: Commands,
}
#[derive(Clap)]
pub enum Commands {
    Create(CreateOptions),
}
#[derive(Clap)]
pub struct CreateOptions {
    pub host: String,
    pub database_name: String,
    pub master_user_name: String,
    pub owner_user_name: String,
}
