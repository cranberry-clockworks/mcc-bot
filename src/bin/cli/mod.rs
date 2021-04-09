use clap::Clap;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Options {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Clap)]
pub enum Commands {
    Create(CreateOptions)
}
#[derive(Clap)]
pub struct CreateOptions {
    arg3: String
}