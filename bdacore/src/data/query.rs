use crate::logic;
use bdaindex::bql;
use bdaindex::bql::BQL;
use bdaproto::{DelResourcesRequest, GetResourcesRequest};

use super::EntityKind;

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub kind: EntityKind,
    pub ast: BQL,
}

pub fn new(kind: EntityKind, ast: BQL) -> Query {
    Query::new(kind, ast)
}

impl Query {
    pub fn new(kind: EntityKind, ast: BQL) -> Query {
        Query { kind, ast }
    }
    pub fn from_get_resources_request(request: &GetResourcesRequest) -> Result<Query, String> {
        bql_join(vec![
            bql_from_namespaces(&request.namespaces),
            bql_from_version(&request.version),
            bql_from_kinds(&request.kinds),
            bql_from_names(&request.names),
            bql_from_str(&request.bql),
        ])
        .ok_or_else(|| format!("could not build query from request {:?}", request))
        .and_then(|ref bql| {
            Ok(Query {
                kind: EntityKind::Resource,
                ast: bql::from_str(bql)?,
            })
        })
    }

    pub fn from_del_resources_request(request: &DelResourcesRequest) -> Result<Query, String> {
        bql_join(vec![
            bql_from_namespaces(&request.namespaces),
            bql_from_version(&request.version),
            bql_from_kinds(&request.kinds),
            bql_from_names(&request.names),
            bql_from_str(&request.bql),
        ])
        .ok_or_else(|| format!("could not build query from request {:?}", request))
        .and_then(|ref bql| {
            Ok(Query {
                kind: EntityKind::Resource,
                ast: bql::from_str(bql)?,
            })
        })
    }
}

pub fn bql_from_namespaces(s: &str) -> Option<String> {
    let mut ns: Vec<String> = Vec::new();
    for n in s.split(",") {
        if n == "" || n == "all" {
            return None; //does not filter namespaces
        } else {
            ns.push(format!("'{}'", n.to_lowercase().replace("'", "\\'")));
        }
    }
    Some(format!(".namespace@any[{}]", ns.join(",")))
}
pub fn bql_from_version(s: &str) -> Option<String> {
    let version = match s.split(",").next() {
        Some(v) if v != "" => v.to_lowercase().replace("'", "\\'"),
        _ => logic::DEFAULT_VERSION.to_string(),
    };
    Some(format!(".version=='{}'", version))
}
pub fn bql_from_kinds(s: &str) -> Option<String> {
    let mut ns: Vec<String> = Vec::new();
    for n in s.split(",") {
        if n == "" || n == "all" {
            return None; //does not filter kinds
        } else {
            ns.push(format!(".{}", n.to_lowercase()));
        }
    }
    Some(format!("{}", ns.join("||")))
}
pub fn bql_from_names(s: &str) -> Option<String> {
    let mut ns: Vec<String> = Vec::new();
    for n in s.split(",") {
        if n == "" {
            return None;
        } else {
            ns.push(format!(".name=='{}'", n.to_lowercase().replace("'", "\'")));
        }
    }
    Some(format!("{}", ns.join("||")))
}

pub fn bql_from_str(s: &str) -> Option<String> {
    if s == "" {
        None
    } else {
        Some(s.to_string())
    }
}
pub fn bql_join(expressions: Vec<Option<String>>) -> Option<String> {
    let mut vs: Vec<String> = Vec::new();
    for e in expressions {
        match e {
            Some(s) => vs.push(format!("( {} )", s)),
            None => {}
        }
    }
    if vs.len() == 0 {
        None
    } else if vs.len() == 1 {
        Some(vs[0].clone())
    } else {
        Some(vs.join("&&"))
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_bdaql_and() {
        let namespaces = bql_from_namespaces("");
        assert_eq!(namespaces, None);
        let namespaces = bql_from_namespaces("all");
        assert_eq!(namespaces, None);
        let namespaces = bql_from_namespaces("anamespace,all");
        assert_eq!(namespaces, None);

        let version = bql_from_version("");
        assert_eq!(
            version,
            Some(format!(
                ".version=='{}'",
                logic::DEFAULT_VERSION.to_string()
            ))
        );

        let kinds = bql_from_kinds("");
        assert_eq!(kinds, None);
        let kinds = bql_from_kinds("function");
        assert_eq!(kinds, Some(".function".to_string()));
        let kinds = bql_from_kinds("function,runtime.container");
        assert_eq!(kinds, Some(".function||.runtime.container".to_string()));

        let names = bql_from_names("");
        assert_eq!(names, None);
        let names = bql_from_names("aname'");
        assert_eq!(names, Some(".name=='aname\''".to_string()));
        let names = bql_from_names("namea,nameb");
        assert_eq!(names, Some(".name=='namea'||.name=='nameb'".to_string()));

        let bdaql = bql_from_str("");
        assert_eq!(bdaql, None);
        let bdaql = bql_from_str(".name");
        assert_eq!(bdaql, Some(".name".to_string()));

        let namespaces = bql_from_namespaces("ns1,ns2");
        assert_eq!(namespaces, Some(".namespace@any['ns1','ns2']".to_string()));
        let version = bql_from_version("");
        assert_eq!(
            version,
            Some(format!(
                ".version=='{}'",
                logic::DEFAULT_VERSION.to_string()
            ))
        );
        let kinds = bql_from_kinds("function,runtime.container");
        assert_eq!(kinds, Some(".function||.runtime.container".to_string()));
        let bdaql = bql_from_str("");
        assert_eq!(bdaql, None);
        let and = bql_join(vec![namespaces, version, kinds, bdaql]);
        assert_eq!(
            and,
            Some(format!(
                "( {} )&&( {} )&&( {} )",
                ".namespace@any['ns1','ns2']".to_string(),
                format!(".version=='{}'", logic::DEFAULT_VERSION.to_string()),
                ".function||.runtime.container".to_string()
            ))
        );
        let namespaces = bql_from_namespaces("ns1,ns2");
        assert_eq!(namespaces, Some(".namespace@any['ns1','ns2']".to_string()));
        let and = bql_join(vec![namespaces, None, None, None]);
        assert_eq!(
            and,
            Some(format!("( {} )", ".namespace@any['ns1','ns2']".to_string()))
        );
        let and = bql_join(vec![None, None, None, None]);
        assert_eq!(and, None)
    }
}
