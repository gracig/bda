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
}
