use bdaproto::Resource;

pub fn defaults(r: &mut Resource) {
    default_string_if_empty(&mut r.namespace, "default");
    default_string_if_empty(&mut r.revision, "latest");
}

fn default_string_if_empty(v: &mut String, d: &str) {
    if v == "" {
        *v = d.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use bdaproto::*;

    #[test]
    fn test_default_string_if_empty() {
        let mut s = String::new();
        super::default_string_if_empty(&mut s, "default");
        assert_eq!(s, "default")
    }
    #[test]
    fn test_defaults() {
        let expected = Resource {
            name: String::from(""),
            description: String::from(""),
            namespace: String::from("default"),
            revision: String::from("latest"),
            tags: Vec::new(),
            resource_kind: None,
        };
        let mut r = expected.clone();
        r.namespace = "".to_owned();
        r.revision = "".to_owned();
        assert_ne!(r, expected);
        super::defaults(&mut r);
        assert_eq!(r, expected);

        //use bdaproto::resource::*;
        //use std::collections::BTreeMap;
        //r.resource_kind = Some(ResourceKind::Variables(Variables {
        //    data: Some(prost_types::Struct {
        //        fields: BTreeMap::from([
        //            (
        //                "Mercury".to_owned(),
        //                prost_types::Value {
        //                    kind: Some(prost_types::value::Kind::NumberValue(4.0)),
        //                },
        //            ),
        //            (
        //                "Venus".to_owned(),
        //                prost_types::Value {
        //                    kind: Some(prost_types::value::Kind::NumberValue(0.7)),
        //                },
        //            ),
        //            (
        //                "Earth".to_owned(),
        //                prost_types::Value {
        //                    kind: Some(prost_types::value::Kind::NumberValue(1.0)),
        //                },
        //            ),
        //            (
        //                "Mars".to_owned(),
        //                prost_types::Value {
        //                    kind: Some(prost_types::value::Kind::NumberValue(1.5)),
        //                },
        //            ),
        //        ]),
        //    }),
        //}));
    }
}
