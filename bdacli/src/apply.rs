use bdaproto::{bda_client::BdaClient, Resource};
use clap::Args;
use std::{
    error::Error,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};
use tonic::{transport::Channel, Request};
use yaml_rust::{YamlEmitter, YamlLoader};

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
    client: &mut BdaClient<Channel>,
    cfg: &crate::apply::Config,
) -> Result<(), Box<dyn Error>> {
    if cfg.debug {
        eprintln!("{:?}", cfg);
    }
    for resource in collect_resources(cfg)? {
        client
            .put_resource(Request::new(bdaproto::PutResourceRequest {
                resource: Some(resource),
            }))
            .await?;
    }

    Ok(())
}
fn collect_resources(cfg: &Config) -> Result<Vec<Resource>, Box<dyn Error>> {
    let mut resources = cfg.json.clone();
    if !cfg.file.is_empty() {
        for f in &cfg.file {
            match f.extension() {
                None => continue,
                Some(x) if x == "json" => {
                    if cfg.debug {
                        eprintln!("Reading JSON file: {}...", f.to_str().unwrap_or("None"))
                    }
                    File::open(f)
                        .and_then(|f| Ok(BufReader::new(f)))
                        .and_then(|r| Ok(serde_json::from_reader(r)?))
                        .and_then(|r| {
                            if cfg.debug {
                                eprintln!("    Adding resource: {:?}...", r)
                            }
                            Ok(resources.push(r))
                        })?;
                }
                Some(x) if x == "yaml" || x == "yml" => {
                    if cfg.debug {
                        eprintln!("Reading YAML file: {}...", f.to_str().unwrap_or("None"))
                    }
                    for ref y in fs::read_to_string(f)
                        .and_then(|ref s| Ok(YamlLoader::load_from_str(s)))??
                    {
                        let mut out_str = String::new();
                        let mut emitter = YamlEmitter::new(&mut out_str);
                        emitter.dump(y).unwrap();
                        let r: Resource = serde_yaml::from_str(&out_str)?;
                        if cfg.debug {
                            eprintln!("    Adding resource: {:?}...", r)
                        }
                        resources.push(r);
                    }
                }
                _ => eprintln!(
                    "Ignoring file: {}. Only extensions json|yaml|yml are accepted",
                    f.to_str().unwrap_or("None")
                ),
            }
        }
    }
    Ok(resources)
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
