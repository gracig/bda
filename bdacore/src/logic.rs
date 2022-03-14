use bdaproto::resource::ResourceKind;
use bdaproto::runtime::RuntimeKind;
use bdaproto::{Container, DelResourceRequest, Function, GetResourceRequest, Resource, Runtime};

const FUNCTION_KIND: &str = "function";
const RUNTIME_CONTAINER_KIND: &str = "runtime.container";
const KINDS: [&str; 2] = [FUNCTION_KIND, RUNTIME_CONTAINER_KIND];
pub const DEFAULT_NAMESPACE: &str = "default";
pub const DEFAULT_REVISION: &str = "latest";
pub const DEFAULT_DOCKERFILE: &str = "Dockerfile";

pub fn new_resource_runtime_container(name: &str) -> Resource {
    new_resource(
        name,
        Some(ResourceKind::Runtime(new_runtime(Some(
            RuntimeKind::Container(new_container()),
        )))),
    )
}

pub fn new_resource_function(name: &str) -> Resource {
    new_resource(name, Some(ResourceKind::Function(new_function())))
}

fn new_container() -> Container {
    Container {
        dockerfile: String::new(),
    }
}

fn new_runtime(runtime_kind: Option<RuntimeKind>) -> Runtime {
    Runtime {
        capabilities: Vec::new(),
        runtime_kind: runtime_kind,
    }
}

fn new_function() -> Function {
    Function {
        inputs: Vec::new(),
        outputs: Vec::new(),
        base_command: Vec::new(),
        runtime_capabilities: Vec::new(),
    }
}

fn new_resource(name: &str, kind: Option<ResourceKind>) -> Resource {
    let mut r = Resource {
        version: String::new(),
        namespace: String::new(),
        name: name.to_owned(),
        description: String::new(),
        tags: Vec::new(),
        attributes: None,
        resource_kind: kind,
    };
    defaults(&mut r);
    r
}

pub fn defaults(r: &mut Resource) {
    default_string_if_empty(&mut r.namespace, DEFAULT_NAMESPACE);
    default_string_if_empty(&mut r.version, DEFAULT_REVISION);
    if let Some(ResourceKind::Runtime(r)) = &mut r.resource_kind {
        if let Some(RuntimeKind::Container(c)) = &mut r.runtime_kind {
            default_string_if_empty(&mut c.dockerfile, DEFAULT_DOCKERFILE);
        }
    }
}

fn default_string_if_empty(v: &mut String, d: &str) {
    if v == "" {
        *v = d.to_owned()
    }
}

pub fn resource_kinds_iter() -> impl Iterator<Item = String> {
    KINDS.into_iter().map(|x| x.to_string())
}

pub fn resource_kind_to_string(r: &Resource) -> Option<String> {
    match r.resource_kind.as_ref()? {
        ResourceKind::Function(_) => Some(FUNCTION_KIND.to_string()),
        ResourceKind::Runtime(rt) => match rt.runtime_kind.as_ref()? {
            RuntimeKind::Container(_) => Some(RUNTIME_CONTAINER_KIND.to_string()),
        },
    }
}

pub fn resource_id_builder(
    version: &str,
    namespace: &str,
    kind: &str,
    name: &str,
) -> Result<String, String> {
    Ok(format!("/{}/{}/{}/{}", version, namespace, kind, name))
}

pub fn resource_id_from_get_request(r: &GetResourceRequest) -> Result<String, String> {
    return resource_id_builder(&r.version, &r.namespace, &r.kind, &r.name);
}
pub fn resource_id_from_del_request(r: &DelResourceRequest) -> Result<String, String> {
    return resource_id_builder(&r.version, &r.namespace, &r.kind, &r.name);
}

pub fn resource_id(r: &Resource) -> Result<String, String> {
    return resource_id_builder(
        &r.version,
        &r.namespace,
        resource_kind_to_string(r)
            .ok_or("resource kind not specified".to_string())?
            .as_str(),
        &r.name,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_string_if_empty() {
        let mut s = String::new();
        super::default_string_if_empty(&mut s, "default");
        assert_eq!(s, "default")
    }
    #[test]
    fn test_defaults() {
        let a = new_resource_function("function");
        assert_eq!(a.namespace, DEFAULT_NAMESPACE);
        assert_eq!(a.version, DEFAULT_REVISION);
        let a = new_resource_runtime_container("container");
        if let Some(ResourceKind::Runtime(r)) = a.resource_kind {
            if let Some(RuntimeKind::Container(c)) = r.runtime_kind {
                assert_eq!(c.dockerfile, DEFAULT_DOCKERFILE);
            }
        }
    }
}
