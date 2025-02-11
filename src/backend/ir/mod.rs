use core::fmt;

use crate::reporter::comp;

mod helper;
pub mod flags;

#[derive(Debug, Clone, PartialEq)]
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
    Struct {
        path: String,
        fields: Vec<(String, DataType)>,
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
            DataType::Struct { path, fields } => {
                write!(f, "struct {} {{\n", path)?;
                for (name, t) in fields {
                    write!(f, "    {}: {},\n", name, t)?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(u64),
    Float(f64),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Shl,
    Shr,
    And
}

#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
    Local {
        offset: i16,
        v_type: DataType,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub mangled: String,
    pub metadata: String,
    pub name: String,
    pub arguments: Vec<(String, DataType)>,
    pub return_type: DataType,
    pub flags: u16,
    pub body: Vec<Statement>,
}
impl Function {
    pub fn new(name: String, arguments: Vec<(String, DataType)>, return_type: DataType, flags: u16, body: Vec<Statement>) -> Function {
        let metadata = helper::mangling::mangle(name.clone(), arguments.clone(), return_type.clone());
        Function {
            mangled: if (flags & flags::FunctionModifer::NoMangle as u16) != 0 { name.clone() } else { helper::mangling::hash(metadata.clone()) },
            metadata,
            name,
            arguments,
            return_type,
            flags,
            body,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct FunctionCall {
    pub reference: Function,
    pub arguments: Vec<Expr>,
}
impl std::fmt::Debug for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Call {{ name: {}, arguments: {:#?} }}", self.reference.name, self.arguments)
    }
}



#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Reference(Reference),
    Call(FunctionCall),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn infer_type(&mut self, enforced_cast: Option<DataType>) -> DataType {
        match self {
            Expr::Literal(literal) => {
                if let Some(enforced_cast) = enforced_cast {
                    return enforced_cast;
                }
                match literal {
                    Literal::Int(_) => DataType::Scalar { size: 4, signed: true },
                    Literal::Float(_) => DataType::Scalar { size: 8, signed: true },
                    Literal::Bool(_) => DataType::Scalar { size: 1, signed: false },
                    Literal::Str(_) => DataType::Pointer { target: Box::new(DataType::Scalar { size: 1, signed: false }) },
                }
            }
            Expr::Reference(reference) => {
                match reference {
                    Reference::Local { v_type, .. } => v_type.clone(),
                }
            }
            Expr::Binary { left, right, .. } => {
                let left = left.infer_type(None);
                let right = right.infer_type(None);
                if left != right {
                    todo!("Type mismatch");
                }
                left
            }
            Expr::Call(call) => {
                call.reference.return_type.clone()
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expr(Expr),
    Return(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub types_def: std::collections::HashMap<String, DataType>, // <Plain Name, Type>
    pub functions: std::collections::HashMap<String, Function> // <Plain Name, Function>
}
impl Module {
    pub fn new() -> Module {
        Module {
            types_def: vec![
                ("void".to_string(), DataType::Scalar { size: 0, signed: false }),
                ("i8".to_string(), DataType::Scalar { size: 1, signed: true }),
                ("u8".to_string(), DataType::Scalar { size: 1, signed: false }),
                ("i16".to_string(), DataType::Scalar { size: 2, signed: true }),
                ("u16".to_string(), DataType::Scalar { size: 2, signed: false }),
                ("i32".to_string(), DataType::Scalar { size: 4, signed: true }),
                ("u32".to_string(), DataType::Scalar { size: 4, signed: false }),
                ("i64".to_string(), DataType::Scalar { size: 8, signed: true }),
                ("u64".to_string(), DataType::Scalar { size: 8, signed: false }),
            ].into_iter().collect::<std::collections::HashMap<_, _>>(),
            functions: std::collections::HashMap::new()
        }
    }
    pub fn push<T: Into<Function>>(&mut self, function: T) -> Result<(), comp::CompilerError> {
        let function: Function = function.into();
        if self.functions.contains_key(&function.metadata) {
            return Err(comp::CompilerError::AlreadyDefinedFunction{name: function.metadata});
        }
        self.functions.insert(function.name.clone(), function);
        Ok(())
    }

    pub fn resolve_type(&self, name: String) -> Result<DataType, comp::CompilerError> {
        match self.types_def.get(&name) {
            Some(t) => Ok(t.clone()),
            None => Err(comp::CompilerError::TypeNotFound{name})
        }
    }

    pub fn resolve_call(&self, name: String, args: Vec<Expr>) -> Result<Expr, comp::CompilerError> {
        for (_, function) in self.functions.iter() {
            if function.name == name {
                if function.arguments.len() != args.len() {
                    continue;
                }
                let mut valid = true;
                for (i, arg) in args.iter().enumerate() {
                    if function.arguments[i].1 != arg.clone().infer_type(Some(function.arguments[i].1.clone())) {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    return Ok(Expr::Call(FunctionCall {
                        reference: function.clone(),
                        arguments: args,
                    }));
                }
            }
        }
        Err(comp::CompilerError::FunctionNotFound{name})
    }
}