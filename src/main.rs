mod application;
mod external;

use clap::Parser;

use crate::application::usecases::consumer_api;
use crate::external::cli::clap::Cli;
use crate::external::database::migrations;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    consumer_api::execute(cli.api_name).await;

    if cli.migration.unwrap() {
        migrations::execute().expect("Error to perform migrations!");
    }
}
