use bdaproto::bda_client::BdaClient;
use clap::{Args, Parser, Subcommand};
use std::error::Error;
use tonic::{transport::Endpoint, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::parse();

    match cfg.command {
        Command::Datastore(cfg) => {
            eprintln!("Connecting to datastore on {:?}", cfg.endpoint.uri());
            let mut client = BdaClient::connect(cfg.endpoint).await?;
            println!(
                "kinds {:?}",
                client
                    .get_kinds(Request::new(bdaproto::GetKindsRequest {}))
                    .await?
            );
        }
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Config {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(author, version, about, long_about = None)]
    Datastore(DatastoreConfig),
}
#[derive(Args, Debug)]
#[clap(author, version, about, long_about = None)]
struct DatastoreConfig {
    #[clap(short, long, default_value = "http://127.0.0.1:7000")]
    endpoint: Endpoint,
}
