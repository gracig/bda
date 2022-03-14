use bdaproto::{bda_client::BdaClient, Resource};
use clap::Args;
use std::{error::Error, path::PathBuf};
use tonic::transport::Channel;

#[derive(Args, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, min_values = 1, required_unless_present_all = ["json"])]
    pub file: Vec<PathBuf>,
    #[clap(short, long, min_values = 1, required_unless_present_all = ["file"])]
    pub json: Vec<Resource>,
    #[clap(short, long)]
    pub debug: bool,
}

pub async fn cmd(
    _client: &mut BdaClient<Channel>,
    cfg: &crate::apply::Config,
) -> Result<(), Box<dyn Error>> {
    if cfg.debug {
        eprintln!("{:?}", cfg);
    }
    Ok(())
}

#[cfg(test)]
mod test_super {
    use super::*;
    use clap::StructOpt;
    #[test]
    fn test_apply_miss() {
        if let Err(_) = crate::Config::try_parse_from(["bdacli", "apply"]) {
        } else {
            assert!(false, "parse should have generated an error")
        }
    }

    #[test]
    fn test_apply_file() {
        let exp = Config {
            file: vec![PathBuf::from("file.json")],
            json: vec![],
            debug: false,
        };
        let cfg = crate::Config::try_parse_from(["bdacli", "apply", "-f", "file.json"]).unwrap();
        assert_eq!(cfg.command, crate::Command::Apply(exp))
    }

    #[test]
    fn test_apply_json() {
        let exp = Config {
            file: vec![],
            json: vec![Resource {
                version: String::new(),
                namespace: String::new(),
                name: String::from("name"),
                description: String::new(),
                tags: vec![],
                attributes: None,
                resource_kind: None,
            }],
            debug: false,
        };
        let cfg =
            crate::Config::try_parse_from(["bdacli", "apply", "-j", r#"{"name":"name"}"#]).unwrap();
        assert_eq!(cfg.command, crate::Command::Apply(exp))
    }
    #[test]

    fn test_apply_both() {
        let exp = Config {
            file: vec![PathBuf::from("file.json")],
            json: vec![Resource {
                version: String::new(),
                namespace: String::new(),
                name: String::from("name"),
                description: String::new(),
                tags: vec![],
                attributes: None,
                resource_kind: None,
            }],
            debug: false,
        };
        let cfg = crate::Config::try_parse_from([
            "bdacli",
            "apply",
            "-f",
            "file.json",
            "-j",
            r#"{"name":"name"}"#,
        ])
        .unwrap();
        assert_eq!(cfg.command, crate::Command::Apply(exp))
    }
}
