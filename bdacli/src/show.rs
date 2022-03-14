use bdaproto::{bda_client::BdaClient, GetKindsRequest, GetNamespacesRequest, GetVersionsRequest};
use clap::{ArgEnum, Args};
use std::error::Error;
use tonic::{transport::Channel, Request};

#[derive(Args, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(arg_enum, required = true, index = 1)]
    pub show_type: ShowType,
    #[clap(short, long)]
    pub debug: bool,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum ShowType {
    Kinds,
    Namespaces,
    Versions,
}

pub async fn cmd(
    client: &mut BdaClient<Channel>,
    cfg: &crate::show::Config,
) -> Result<(), Box<dyn Error>> {
    if cfg.debug {
        eprintln!("{:?}", cfg);
    }
    match cfg.show_type {
        ShowType::Kinds => show(
            &client
                .get_kinds(Request::new(GetKindsRequest {}))
                .await?
                .get_ref()
                .kinds,
        ),
        ShowType::Namespaces => show(
            &client
                .get_namespaces(Request::new(GetNamespacesRequest {}))
                .await?
                .get_ref()
                .namespaces,
        ),
        ShowType::Versions => show(
            &client
                .get_versions(Request::new(GetVersionsRequest {}))
                .await?
                .get_ref()
                .versions,
        ),
    }
    Ok(())
}
fn show(items: &Vec<String>) {
    for item in items {
        println!("{}", item)
    }
}
