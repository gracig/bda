pub mod apply;
pub mod get;
pub mod show;

use std::error::Error;

use bdaproto::bda_client::BdaClient;
use clap::{Args, Parser, Subcommand};
use tonic::transport::Channel;
use url::Url;

const DEFAULT_DATASTORE_ENDPOINT: &str = "http://127.0.0.1:7000";
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(subcommand)]
    pub command: Command,
    #[clap(flatten)]
    pub datastore_conn: DatastoreConn,
}
#[derive(Debug, Args)]
pub struct DatastoreConn {
    #[clap(short, long, default_value = DEFAULT_DATASTORE_ENDPOINT)]
    pub endpoint: Url,
}
#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    Get(get::Config),
    Show(show::Config),
    Apply(apply::Config),
}

pub async fn connect(cfg: &DatastoreConn) -> Result<BdaClient<Channel>, Box<dyn Error>> {
    Ok(BdaClient::connect(cfg.endpoint.to_string()).await?)
}
