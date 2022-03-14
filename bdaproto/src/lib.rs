use std::str::FromStr;

include!("bda.rs");
include!("bda.serde.rs");
include!("google.api.rs");
include!("google.protobuf.rs");

impl FromStr for Resource {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
