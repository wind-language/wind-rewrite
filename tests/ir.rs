use wind::backend::ir;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ir_tree_initialization() {
        let mut tree = ir::Module::new();
        let _ = tree.push(
            ir::Function::new(
                "add".to_string(),
                vec![
                    ("a".to_string(), ir::DataType::Scalar { size: 4, signed: true }),
                    ("b".to_string(), ir::DataType::Scalar { size: 4, signed: true }),
                ],
                ir::DataType::Scalar { size: 4, signed: true },
                0,
                vec![]
            )
        );
        let _ = tree.push(
            ir::Function::new(
                "main".to_string(),
                vec![],
                ir::DataType::Scalar { size: 4, signed: true },
                ir::flags::FunctionModifer::NoMangle as u16,
                vec![]
            )
        );
        println!("{:#?}", tree);
    }
}