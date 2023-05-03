mod application;
mod domain;
mod external;

use clap::Parser;

use crate::application::usecases::consumer_api;
use crate::external::cli::clap::Cli;
use crate::external::database::migrations;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.migration.is_some() {
        migrations::execute().expect("Error to perform migrations!");
    }

    consumer_api::execute(cli.api_name, cli.save_request.is_some()).await;
}
