use bdacli::{self, apply, get, show, Command};
use clap::Parser;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ref cfg = bdacli::Config::parse();
    let mut client = bdacli::connect(&cfg.datastore_conn).await?;
    match cfg.command {
        Command::Get(ref cfg) => get::cmd(&mut client, cfg).await?,
        Command::Show(ref cfg) => show::cmd(&mut client, cfg).await?,
        Command::Apply(ref cfg) => apply::cmd(&mut client, cfg).await?,
    }
    Ok(())
}
