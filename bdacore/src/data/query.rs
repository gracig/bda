use crate::logic;
use bdaproto::{DelResourcesRequest, GetResourcesRequest};
use bdaql::Ast;

use super::EntityKind;

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub kind: EntityKind,
    pub ast: Ast,
}

pub fn new(kind: EntityKind, ast: Ast) -> Query {
    Query::new(kind, ast)
}

impl Query {
    pub fn new(kind: EntityKind, ast: Ast) -> Query {
        Query { kind, ast }
    }
    pub fn from_get_resources_request(request: &GetResourcesRequest) -> Result<Query, String> {
        bdaql_conjunction(vec![
            bdaql_from_namespaces(&request.namespaces),
            bdaql_from_version(&request.version),
            bdaql_from_kinds(&request.kinds),
            bdaql_from_bql(&request.bql),
        ])
        .ok_or_else(|| format!("could not build query from request {:?}", request))
        .and_then(|ref bql| {
            Ok(Query {
                kind: EntityKind::Resource,
                ast: bdaql::from_str(bql)?,
            })
        })
    }

    pub fn from_del_resources_request(request: &DelResourcesRequest) -> Result<Query, String> {
        bdaql_conjunction(vec![
            bdaql_from_namespaces(&request.namespaces),
            bdaql_from_version(&request.version),
            bdaql_from_kinds(&request.kinds),
            bdaql_from_bql(&request.bql),
        ])
        .ok_or_else(|| format!("could not build query from request {:?}", request))
        .and_then(|ref bql| {
            Ok(Query {
                kind: EntityKind::Resource,
                ast: bdaql::from_str(bql)?,
            })
        })
    }
}

fn bdaql_from_namespaces(s: &str) -> Option<String> {
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
fn bdaql_from_version(s: &str) -> Option<String> {
    let version = match s.split(",").next() {
        Some(v) if v != "" => v.to_lowercase().replace("'", "\\'"),
        _ => logic::DEFAULT_REVISION.to_string(),
    };
    Some(format!(".version=='{}'", version))
}
fn bdaql_from_kinds(s: &str) -> Option<String> {
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
fn bdaql_from_bql(s: &str) -> Option<String> {
    if s == "" {
        None
    } else {
        Some(s.to_string())
    }
}
fn bdaql_conjunction(expressions: Vec<Option<String>>) -> Option<String> {
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
        let namespaces = bdaql_from_namespaces("");
        assert_eq!(namespaces, None);
        let namespaces = bdaql_from_namespaces("all");
        assert_eq!(namespaces, None);
        let namespaces = bdaql_from_namespaces("anamespace,all");
        assert_eq!(namespaces, None);

        let version = bdaql_from_version("");
        assert_eq!(
            version,
            Some(format!(
                ".version=='{}'",
                logic::DEFAULT_REVISION.to_string()
            ))
        );

        let kinds = bdaql_from_kinds("");
        assert_eq!(kinds, None);
        let kinds = bdaql_from_kinds("function");
        assert_eq!(kinds, Some(".function".to_string()));
        let kinds = bdaql_from_kinds("function,runtime.container");
        assert_eq!(kinds, Some(".function||.runtime.container".to_string()));

        let bdaql = bdaql_from_bql("");
        assert_eq!(bdaql, None);
        let bdaql = bdaql_from_bql(".name");
        assert_eq!(bdaql, Some(".name".to_string()));

        let namespaces = bdaql_from_namespaces("ns1,ns2");
        assert_eq!(namespaces, Some(".namespace@any['ns1','ns2']".to_string()));
        let version = bdaql_from_version("");
        assert_eq!(
            version,
            Some(format!(
                ".version=='{}'",
                logic::DEFAULT_REVISION.to_string()
            ))
        );
        let kinds = bdaql_from_kinds("function,runtime.container");
        assert_eq!(kinds, Some(".function||.runtime.container".to_string()));
        let bdaql = bdaql_from_bql("");
        assert_eq!(bdaql, None);
        let and = bdaql_conjunction(vec![namespaces, version, kinds, bdaql]);
        assert_eq!(
            and,
            Some(format!(
                "( {} )&&( {} )&&( {} )",
                ".namespace@any['ns1','ns2']".to_string(),
                format!(".version=='{}'", logic::DEFAULT_REVISION.to_string()),
                ".function||.runtime.container".to_string()
            ))
        );
        let namespaces = bdaql_from_namespaces("ns1,ns2");
        assert_eq!(namespaces, Some(".namespace@any['ns1','ns2']".to_string()));
        let and = bdaql_conjunction(vec![namespaces, None, None, None]);
        assert_eq!(
            and,
            Some(format!("( {} )", ".namespace@any['ns1','ns2']".to_string()))
        );
        let and = bdaql_conjunction(vec![None, None, None, None]);
        assert_eq!(and, None)
    }
}
