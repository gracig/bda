use bdaproto::resource::ResourceKind;
use bdaproto::runtime::RuntimeKind;
use bdaproto::Resource;

pub fn defaults(r: &mut Resource) {
    default_string_if_empty(&mut r.namespace, "default");
    default_string_if_empty(&mut r.revision, "latest");
}

pub fn resource_kind_as_string(r: &Resource) -> Option<String> {
    match r.resource_kind.as_ref()? {
        ResourceKind::Function(_) => Some("function".to_owned()),
        ResourceKind::Runtime(rt) => match rt.runtime_kind.as_ref()? {
            RuntimeKind::Container(_) => Some("runtime.container".to_owned()),
        },
    }
}

fn default_string_if_empty(v: &mut String, d: &str) {
    if v == "" {
        *v = d.to_owned()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_default_string_if_empty() {
        let mut s = String::new();
        super::default_string_if_empty(&mut s, "default");
        assert_eq!(s, "default")
    }
    #[test]
    fn test_defaults() {
        let expected: bdaproto::Resource = serde_json::from_value(serde_json::json!({
            "name": "resource_name",
            "description": "description",
            "namespace": "default",
            "revision": "latest",
            "tags": ["tag1", "tag2"],
        })).unwrap();
        let mut got = expected.clone();
        got.namespace = "".to_owned();
        got.revision = "".to_owned();
        assert_ne!(got, expected);
        super::defaults(&mut got);
        assert_eq!(got, expected);
    }
}
