use bdaproto::Resource;

pub fn resource_id_builder(
    version: &str,
    namespace: &str,
    kind: &str,
    name: &str,
) -> Result<String, String> {
    Ok(format!("/{}/{}/{}/{}", version, namespace, kind, name))
}

pub fn resource_id(r: &Resource) -> Result<String, String> {
    return resource_id_builder(
        &r.version,
        &r.namespace,
        r.resource_kind_str()
            .ok_or("resource kind not specified")
            .unwrap()
            .as_str(),
        &r.name,
    );
}
