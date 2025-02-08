use crate::reporter::comp;

mod helper;
pub mod flags;

#[derive(Debug, Clone)]
pub enum DataType {
    Scalar {
        size: i16,
        signed: bool,
    },
    Pointer {
        target: Box<DataType>,
    },
    Array {
        target: Box<DataType>,
        capacity: i16,
    },
}
impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataType::Scalar { size, signed } => {
                if *size == 0 {
                    return write!(f, "void");
                }
                write!(f, "{}{}", if *signed { "i" } else { "u" }, size*8)
            }
            DataType::Pointer { target } => {
                write!(f, "*{}", target)
            }
            DataType::Array { target, capacity } => {
                write!(f, "[{}; {}]", target, capacity)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Reference {
    Local {
        offset: i16,
        v_type: DataType,
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub metadata: String,
    pub arguments: Vec<(String, DataType)>,
    pub return_type: DataType,
    pub flags: u16,
    pub body: Vec<Statement>,
}
impl Function {
    pub fn new(name: String, arguments: Vec<(String, DataType)>, return_type: DataType, flags: u16, body: Vec<Statement>) -> Function {
        let metadata = helper::mangling::mangle(name.clone(), arguments.clone(), return_type.clone());
        Function {
            name: if (flags & flags::FunctionModifer::NoMangle as u16) != 0 { name } else { helper::mangling::hash(metadata.clone()) },
            metadata,
            arguments,
            return_type,
            flags,
            body,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Reference(Reference),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        function: String,
        reference: Box<Function>,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expr),
    Return(Expr),
}

#[derive(Debug, Clone)]
pub struct Module {
    pub functions: std::collections::HashMap<String, Function>, // <Metadata, Function>
}
impl Module {
    pub fn new() -> Module {
        Module {
            functions: std::collections::HashMap::new(),
        }
    }
    pub fn push<T: Into<Function>>(&mut self, function: T) -> Result<(), comp::CompilerError> {
        let function = function.into();
        if self.functions.contains_key(&function.metadata) {
            return Err(comp::CompilerError::AlreadyDefinedFunction{name: function.metadata});
        }
        self.functions.insert(function.metadata.clone(), function);
        Ok(())
    }
}