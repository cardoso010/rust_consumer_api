use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of the consumir api
    #[arg(short, long)]
    pub api_name: String,

    /// if it'll execute migrations
    #[arg(short, long)]
    pub migration: Option<bool>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
