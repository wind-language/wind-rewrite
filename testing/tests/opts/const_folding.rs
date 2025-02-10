#[cfg(test)]
mod const_folding_test {
    use wind::backend::{ir, opt};

    #[test]
    pub fn t2_const_fold() {
        let mut tree = ir::Module::new();
        let _ = tree.push(
            ir::Function::new(
                "main".to_string(),
                vec![],
                tree.resolve_type("i32".to_string()).unwrap(),
                ir::flags::FunctionModifer::NoMangle as u16,
                vec![
                    ir::Statement::Return(
                        ir::Expr::Binary {
                            op: ir::BinaryOp::Add,
                            left: Box::new( ir::Expr::Literal( ir::Literal::Int(5) ) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(3) ) )
                        }
                    )
                ]
            )
        );
        let mut opt = opt::PassManager::new();
        opt.add_pass(opt::pipeline::folding::ConstantFolding::new());
        opt.run_all(&mut tree);

        let mut expected_tree = ir::Module::new();
        let _ = expected_tree.push(
            ir::Function::new(
                "main".to_string(),
                vec![],
                tree.resolve_type("i32".to_string()).unwrap(),
                ir::flags::FunctionModifer::NoMangle as u16,
                vec![
                    ir::Statement::Return(
                        ir::Expr::Literal( ir::Literal::Int(8) )
                    )
                ]
            )
        );

        assert_eq!(tree, expected_tree);

    }

    #[test]
    fn t3_const_fold() {
        // triple term
        let mut tree = ir::Module::new();
        let _ = tree.push(
            ir::Function::new(
                "main".to_string(),
                vec![],
                tree.resolve_type("i32".to_string()).unwrap(),
                ir::flags::FunctionModifer::NoMangle as u16,
                vec![
                    ir::Statement::Return(
                        ir::Expr::Binary {
                            op: ir::BinaryOp::Add,
                            left: Box::new(
                                ir::Expr::Binary {
                                    op: ir::BinaryOp::Add,
                                    left: Box::new( ir::Expr::Literal( ir::Literal::Int(5) ) ),
                                    right: Box::new( ir::Expr::Literal( ir::Literal::Int(3) ) )
                                }
                            ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(2) ) )
                        }
                    )
                ]
            )
        );

        let mut opt = opt::PassManager::new();
        opt.add_pass(opt::pipeline::folding::ConstantFolding::new());
        opt.run_all(&mut tree);

        let mut expected_tree = ir::Module::new();
        let _ = expected_tree.push(
            ir::Function::new(
                "main".to_string(),
                vec![],
                tree.resolve_type("i32".to_string()).unwrap(),
                ir::flags::FunctionModifer::NoMangle as u16,
                vec![
                    ir::Statement::Return(
                        ir::Expr::Literal( ir::Literal::Int(10) )
                    )
                ]
            )
        );

        assert_eq!(tree, expected_tree);
    }
}