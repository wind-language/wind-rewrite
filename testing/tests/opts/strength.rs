#[cfg(test)]
mod strength_reduction_test {
    use wind::backend::{ir, opt};

    #[test]
    fn mul_reduction() {
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
                            op: ir::BinaryOp::Mul,
                            left: Box::new( ir::Expr::Reference(ir::Reference::Local { offset: 4, v_type: tree.resolve_type("i32".to_string()).unwrap() }) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(4) ) )
                        }
                    ),
                ]
            )
        );
        let mut opt = opt::PassManager::new();
        opt.add_pass(opt::pipeline::folding::ConstantFolding::new());
        opt.add_pass(opt::pipeline::dead_code::DeadCode::new());
        opt.add_pass(opt::pipeline::strength::Strength::new());
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
                        ir::Expr::Binary {
                            op: ir::BinaryOp::Shl,
                            left: Box::new( ir::Expr::Reference(ir::Reference::Local { offset: 4, v_type: tree.resolve_type("i32".to_string()).unwrap() }) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(2) ) )
                        }
                    ),
                ]
            )
        );

        assert_eq!(tree, expected_tree);
    }

    #[test]
    fn zero_mul_reduction() {
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
                            op: ir::BinaryOp::Mul,
                            left: Box::new( ir::Expr::Reference(ir::Reference::Local { offset: 4, v_type: tree.resolve_type("i32".to_string()).unwrap() }) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(0) ) )
                        }
                    ),
                ]
            )
        );
        let mut opt = opt::PassManager::new();
        opt.add_pass(opt::pipeline::folding::ConstantFolding::new());
        opt.add_pass(opt::pipeline::dead_code::DeadCode::new());
        opt.add_pass(opt::pipeline::strength::Strength::new());
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
                        ir::Expr::Literal( ir::Literal::Int(0) )
                    ),
                ]
            )
        );

        assert_eq!(tree, expected_tree);
    }

    #[test]
    fn div_reduction() {
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
                            op: ir::BinaryOp::Div,
                            left: Box::new( ir::Expr::Reference(ir::Reference::Local { offset: 4, v_type: tree.resolve_type("i32".to_string()).unwrap() }) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(4) ) )
                        }
                    ),
                ]
            )
        );
        let mut opt = opt::PassManager::new();
        opt.add_pass(opt::pipeline::folding::ConstantFolding::new());
        opt.add_pass(opt::pipeline::dead_code::DeadCode::new());
        opt.add_pass(opt::pipeline::strength::Strength::new());
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
                        ir::Expr::Binary {
                            op: ir::BinaryOp::Shr,
                            left: Box::new( ir::Expr::Reference(ir::Reference::Local { offset: 4, v_type: tree.resolve_type("i32".to_string()).unwrap() }) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(2) ) )
                        }
                    ),
                ]
            )
        );

        assert_eq!(tree, expected_tree);
    }

    #[test]
    fn odd_mul_reduction() {
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
                            op: ir::BinaryOp::Mul,
                            left: Box::new( ir::Expr::Reference(ir::Reference::Local { offset: 4, v_type: tree.resolve_type("i32".to_string()).unwrap() }) ),
                            right: Box::new( ir::Expr::Literal( ir::Literal::Int(3) ) )
                        }
                    ),
                ]
            )
        );
        let mut opt = opt::PassManager::new();
        opt.add_pass(opt::pipeline::folding::ConstantFolding::new());
        opt.add_pass(opt::pipeline::dead_code::DeadCode::new());
        opt.add_pass(opt::pipeline::strength::Strength::new());
        opt.run_all(&mut tree);

        println!("{:#?}", tree);

        /* let mut expected_tree = ir::Module::new();
        let _ = expected_tree.push(
            ir::Function::new(
                "main".to_string(),
                vec![],
                tree.resolve_type("i32".to_string()).unwrap(),
                ir::flags::FunctionModifer::NoMangle as u16,
                vec![
                    ir::Statement::Return(
                        ir::Expr::Binary {
                            op: ir::BinaryOp::Add,
                            left: Box::new( ir::Expr::Reference(ir::Reference::Local { offset: 4, v_type: tree.resolve_type("i32".to_string()).unwrap() }) ),
                            right: Box::new( ir::Expr::Binary {
                                op: ir::BinaryOp::Shl,
                                left: Box::new( ir::Expr::Reference(ir::Reference::Local { offset: 4, v_type: tree.resolve_type("i32".to_string()).unwrap() }) ),
                                right: Box::new( ir::Expr::Literal( ir::Literal::Int(1) ) )
                            } )
                        }
                    ),
                ]
            )
        );

        assert_eq!(tree, expected_tree); */
    }
}