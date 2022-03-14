use bdaproto::{bda_client::BdaClient, GetResourcesRequest, Resource};
use clap::Args;
use std::error::Error;
use tonic::{transport::Channel, Request};

#[derive(Args, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(required = false, index = 1, default_value = "")]
    pub kinds: String,
    #[clap(required = false, index = 2, default_value = "")]
    pub names: String,
    #[clap(short, long, default_value = bdacore::logic::DEFAULT_NAMESPACE)]
    pub namespace: String,
    #[clap(short, long, default_value = bdacore::logic::DEFAULT_VERSION)]
    pub version: String,
    #[clap(short, long, default_value = "")]
    pub bql: String,
    #[clap(short, long)]
    pub debug: bool,
}

pub async fn cmd(
    client: &mut BdaClient<Channel>,
    cfg: &crate::get::Config,
) -> Result<(), Box<dyn Error>> {
    if cfg.debug {
        eprintln!("{:?}", cfg);
    }
    let request = Request::new(get_resources_request_from_get_cfg(cfg));
    let response = client.get_resources(request).await?;
    show(&response.get_ref().resources);
    Ok(())
}

fn get_resources_request_from_get_cfg(cfg: &crate::get::Config) -> GetResourcesRequest {
    GetResourcesRequest {
        version: cfg.version.to_string(),
        namespaces: cfg.namespace.to_string(),
        names: cfg.names.to_string(),
        kinds: cfg.kinds.to_string(),
        bql: cfg.bql.to_string(),
    }
}

fn show(rs: &Vec<Resource>) {
    println!("{:#?}", rs)
}
