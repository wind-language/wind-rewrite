use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    INTEGER(i64),
    FLOAT(f64),
    BOOL(bool),
    STRING(String)
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    LITERAL(Literal),
    VARIABLE(Variable),
    BINARY {
      left: Box<Expression>,
      right: Box<Expression>,
      op: String
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ASTNode {
    EXPRESSION(Expression),
    VARIABLE(Variable),
    BODY(Body),
    RETURN(Return),
    FUNCTION(Function)
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    EXPRESSION(Expression),
    RETURN(Expression),
}


type DataType = String;


#[derive(PartialEq, Debug, Clone)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Reference {
    Local {
        offset: i16,
        v_type: DataType,
    }
}
  
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    value: Option<Box<ASTNode>>,
}

impl Return {
    pub fn new(value: Option<ASTNode>) -> Self {
        Self {
            value: value.map(Box::new),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Body {
    statements: Vec<ASTNode>,
    pub consts_table: HashMap<String, ASTNode>,
}

impl Body {
    pub fn new(statements: Vec<ASTNode>) -> Self {
        Self {
            statements,
            consts_table: HashMap::new(),
        }
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub metadata: String,
    pub arguments: Vec<(String, DataType)>,
    pub return_type: DataType,
    pub flags: u16,
    pub body: Vec<Statement>,
}

impl Function {
    pub fn new(name: String, metadata: String, arguments: Vec<(String, DataType)>, 
            return_type: DataType, flags: u16, body: Vec<Statement>) -> Self {
        Self {
            name,
            metadata,
            arguments,
            return_type,
            flags,
            body,
        }
    }
}
