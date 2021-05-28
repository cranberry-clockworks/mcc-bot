use clap::Clap;

/// Manages operations on the service database.
#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Options {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(short, long, default_value = "settings.toml")]
    pub settings: String,
}

#[derive(Clap)]
pub enum Commands {
    /// Create initial database.
    Create(CreateOptions),

    /// Migrate database to last available version.
    Migrate,
}

#[derive(Clap)]
pub struct CreateOptions {
    /// A username of Postgress master accont. Password will be interactively requested.
    #[clap(long, default_value = "postgres")]
    pub master_username: String,
}
