#[cfg(test)]
mod dead_code_test {
    use wind::backend::{ir, opt};

    #[test]
    fn multi_dead_code() {
        let mut tree = ir::Module::new();
        let _ = tree.push(
            ir::Function::new(
                "main".to_string(),
                vec![],
                tree.resolve_type("i32".to_string()).unwrap(),
                ir::flags::FunctionModifer::NoMangle as u16,
                vec![
                    ir::Statement::Expr(
                        ir::Expr::Binary {
                            op: ir::BinaryOp::Add,
                            left: Box::new( ir::Expr::Literal( ir::Literal::Int(2) ) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(3) ) )
                        }
                    ),
                    ir::Statement::Return(
                        ir::Expr::Binary {
                            op: ir::BinaryOp::Add,
                            left: Box::new( ir::Expr::Literal( ir::Literal::Int(5) ) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(3) ) )
                        }
                    ),
                    ir::Statement::Return(
                        ir::Expr::Binary {
                            op: ir::BinaryOp::Add,
                            left: Box::new( ir::Expr::Literal( ir::Literal::Int(6) ) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(1) ) )
                        }
                    )
                ]
            )
        );
        let mut opt = opt::PassManager::new();
        opt.add_pass(opt::pipeline::folding::ConstantFolding::new());
        opt.add_pass(opt::pipeline::dead_code::DeadCode::new());
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
                    ),
                ]
            )
        );

        assert_eq!(tree, expected_tree);
    }

    #[test]
    fn useless_dead_code() {
        let mut tree = ir::Module::new();
        let _ = tree.push(
            ir::Function::new(
                "main".to_string(),
                vec![],
                tree.resolve_type("i32".to_string()).unwrap(),
                ir::flags::FunctionModifer::NoMangle as u16,
                vec![
                    ir::Statement::Expr(
                        ir::Expr::Binary {
                            op: ir::BinaryOp::Add,
                            left: Box::new( ir::Expr::Literal( ir::Literal::Int(9) ) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(1) ) )
                        }
                    ),
                ]
            )
        );
        let mut opt = opt::PassManager::new();
        opt.add_pass(opt::pipeline::folding::ConstantFolding::new());
        opt.add_pass(opt::pipeline::dead_code::DeadCode::new());
        opt.run_all(&mut tree);

        let mut expected_tree = ir::Module::new();
        let _ = expected_tree.push(
            ir::Function::new(
                "main".to_string(),
                vec![],
                tree.resolve_type("i32".to_string()).unwrap(),
                ir::flags::FunctionModifer::NoMangle as u16,
                vec![]
            )
        );

        assert_eq!(tree, expected_tree);
    }
}