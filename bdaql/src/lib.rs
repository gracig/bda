mod ast;
mod parser;
mod scanner;

pub fn from_str(s: &str) -> Result<ast::Ast, String> {
    return parser::parse(s);
}
