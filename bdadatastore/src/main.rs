use std::{error::Error, net::SocketAddr, str::FromStr};

use bdadatastore::BDADatastoreService;
use bdaproto::bda_server::BdaServer;
use clap::Parser;
use tonic::transport::Server;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let ref cfg = Config::parse();
    println!("{:?}", cfg);
    Some(cfg.backend)
        .map(|b| match b {
            DatastoreType::Mem => BDADatastoreService::new_mem(),
        })
        .and_then(|bsvc| Some(BdaServer::new(bsvc)))
        .and_then(|svc| Some(Server::builder().add_service(svc).serve(cfg.address)))
        .ok_or("could not build server")?
        .await?;
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config {
    #[clap(short, long, default_value = "127.0.0.1:7000")]
    address: SocketAddr,
    #[clap(short, long, default_value = "mem")]
    backend: DatastoreType,
}
#[derive(Parser, Debug, Copy, Clone)]
enum DatastoreType {
    Mem,
}
impl FromStr for DatastoreType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == "mem" {
            Ok(DatastoreType::Mem)
        } else {
            Err(format!(
                "{:?} is not reconized as a datastore type. allowed values are: mem ",
                s
            ))
        }
    }
}
