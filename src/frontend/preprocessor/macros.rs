#[derive(Clone, Debug)]
pub enum BodyNode {
    Lexical(String),
    Param(String),
    ArgIteration(Vec<BodyNode>),
}

impl BodyNode {
    pub fn is_param(&self) -> bool {
        matches!(self, BodyNode::Param(_))
    }

    pub fn content(&self) -> Option<&String> {
        match self {
            BodyNode::Lexical(content) | BodyNode::Param(content) => Some(content),
            BodyNode::ArgIteration(_) => None,
        }
    }
}

#[derive(Clone)]
pub struct Macro {
    pub name: String,
    pub body: Vec<BodyNode>,
    pub params: Vec<String>
}

impl Macro {
    pub fn new(name: String, params: Vec<String>, body: Vec<BodyNode>) -> Self {
        Self { name, params, body }
    }

    pub fn uses_param(&self, param: &str) -> bool {
        self.body.iter().any(|node| matches!(node, BodyNode::Param(p) if p == param))
    }
}
