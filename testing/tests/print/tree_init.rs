#[cfg(test)]
mod tree_init_test {
    use wind::backend::ir;

    #[test]
    pub fn tree_initialization() {
        let mut tree = ir::Module::new();
        let _ = tree.push(
            ir::Function::new(
                "add".to_string(),
                vec![
                    ("a".to_string(), tree.resolve_type("i32".to_string()).unwrap()),
                    ("b".to_string(), tree.resolve_type("i32".to_string()).unwrap()),
                ],
                tree.resolve_type("i32".to_string()).unwrap(),
                0,
                vec![
                    ir::Statement::Return(
                        ir::Expr::Binary {
                            op: ir::BinaryOp::Add,
                            left: Box::new(
                                ir::Expr::Reference(ir::Reference::Local { offset: 4, v_type: tree.resolve_type("i32".to_string()).unwrap() })
                            ),
                            right: Box::new(
                                ir::Expr::Reference(ir::Reference::Local { offset: 8, v_type: tree.resolve_type("i32".to_string()).unwrap() })
                            )
                        }
                    )
                ]
            )
        );
        let t_fn_call = tree.resolve_call("add".to_string(), vec![
            ir::Expr::Literal( ir::Literal::Int(5) ),
            ir::Expr::Literal( ir::Literal::Int(3) ),
        ]);
        assert!(t_fn_call.is_ok());
        let _ = tree.push(
            ir::Function::new(
                "main".to_string(),
                vec![],
                tree.resolve_type("i32".to_string()).unwrap(),
                ir::flags::FunctionModifer::NoMangle as u16,
                vec![
                    ir::Statement::Expr(t_fn_call.unwrap())
                ]
            )
        );
    }
}