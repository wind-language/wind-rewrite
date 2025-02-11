use crate::backend::{ir, opt};

struct VisitState {
    func: Option<Box<ir::Function>>
}

pub struct ConstantFolding {
    changed: bool,
    state: VisitState
}

impl ConstantFolding {
    pub fn new() -> Self {
        ConstantFolding {
            changed: false,
            state: VisitState {
                func: None
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut ir::Expr) {
        if let ir::Expr::Binary { op, left, right} = expr {
            if let (ir::Expr::Literal(ir::Literal::Int(l)), ir::Expr::Literal(ir::Literal::Int(r))) = (&**left, &**right) {
                let result = match op {
                    ir::BinaryOp::Add => l + r,
                    ir::BinaryOp::Sub => l - r,
                    ir::BinaryOp::Mul => l * r,
                    ir::BinaryOp::Div => l / r,
                    ir::BinaryOp::Shl => l << r,
                    ir::BinaryOp::Shr => l >> r,
                    ir::BinaryOp::And => l & r,
                };
                self.changed = true;
                *expr = ir::Expr::Literal(ir::Literal::Int(result));
                return; // early return since we have replaced expr.
            }
            self.visit_expr(left);
            self.visit_expr(right);
        } else if let ir::Expr::Call(call) = expr {
            for arg in call.arguments.iter_mut() {
                self.visit_expr(arg);
            }
        }
    }
    

    fn visit_stat(&mut self, stat: &mut ir::Statement) {
        match stat {
            ir::Statement::Expr(expr) => {
                self.visit_expr(expr);
            }
            ir::Statement::Return(expr) => {
                self.visit_expr(expr);
            }
        }
    }
}

impl opt::OptimizationPass for ConstantFolding {
    fn run(&mut self, ir: &mut ir::Module) -> bool {
        self.changed = false;
        for d_fn in ir.functions.iter_mut() {
            self.state.func = Some(Box::new(d_fn.1.clone()));

            for stat in d_fn.1.body.iter_mut() {
                self.visit_stat(stat);
            }

            self.state.func = None;
        }

        self.changed
    }
}