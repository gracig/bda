include!("bda.rs");
include!("bda.serde.rs");
include!("google.api.rs");
include!("google.protobuf.rs");

impl Resource {
    pub fn resource_kind_str(&self) -> Option<String> {
        match self.resource_kind.as_ref()? {
            resource::ResourceKind::Function(_) => Some("function".to_owned()),
            resource::ResourceKind::Runtime(rt) => match rt.runtime_kind.as_ref()? {
                runtime::RuntimeKind::Container(_) => Some("runtime.container".to_owned()),
            },
        }
    }
}
