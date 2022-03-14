use bdadatastore::BDADatastoreService;
use bdaproto::bda_server::BdaServer;
use clap::{ArgEnum, Parser};
use std::{error::Error, net::SocketAddr};
use tonic::transport::Server;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let ref cfg = Config::parse();
    println!("{:?}", cfg);
    Some(cfg.backend)
        .map(|b| match b {
            DatastoreType::Mem => BDADatastoreService::new_mem(),
            DatastoreType::Etcd => todo!(),
            DatastoreType::Redis => todo!(),
            DatastoreType::File => todo!(),
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
    #[clap(arg_enum, short, long, default_value = "mem")]
    backend: DatastoreType,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum DatastoreType {
    Mem,
    Etcd,
    Redis,
    File,
}
