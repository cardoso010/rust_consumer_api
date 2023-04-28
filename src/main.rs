mod application;
mod external;

use clap::Parser;

use crate::application::usecases::consumer_api;
use crate::external::cli::clap::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    consumer_api::execute(cli.api_name).await;
}
