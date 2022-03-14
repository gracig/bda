#[test]
fn test_serde() {
    let r = bdaproto::Resource {
        name: String::from("John Doe"),
        description: String::from("This is a description"),
        namespace: String::from("default"),
        revision: String::from("latest"),
        tags: vec![String::from("tag1"), String::from("tag2")],
        resource_kind: Some(bdaproto::resource::ResourceKind::Runtime(
            bdaproto::Runtime {
                capabilities: vec!["git".to_owned(), "linux".to_owned()],
                runtime_kind: Some(bdaproto::runtime::RuntimeKind::Container(
                    bdaproto::Container {
                        dockerfile: "Dockerfile".to_owned(),
                    },
                )),
            },
        )),
    };
    let j: bdaproto::Resource = serde_json::from_value(serde_json::json!({
        "name": "John Doe",
        "description": "This is a description",
        "namespace": "default",
        "revision": "latest",
        "tags": [ "tag1", "tag2"],
        "runtime": {
            "capabilities": [ "git", "linux" ],
            "container": {
                "dockerfile": "Dockerfile"
            }
        }
    }))
    .unwrap();

    let y: bdaproto::Resource = serde_yaml::from_str(
        r#"---
            name: John Doe
            description: This is a description
            namespace: default
            revision: latest
            tags:
            - tag1
            - tag2
            runtime:
              capabilities:
              - git
              - linux
              container:
                dockerfile: Dockerfile
            "#,
    )
    .unwrap();

    assert_eq!(r, j);
    println!("Hello from struct : {:?}", r);
    println!("Hello from json   : {:?}", j);
    assert_eq!(r, y);
    println!("Hello from yaml   : {:?}", y);
}
