use crate::backend::{ir, opt};

struct VisitState {
    func: Option<Box<ir::Function>>
}

pub struct Strength {
    changed: bool,
    state: VisitState
}

fn is_pow2(n: i64) -> bool {
    n & (n - 1) == 0
}

impl Strength {
    pub fn new() -> Self {
        Strength {
            changed: false,
            state: VisitState {
                func: None
            }
        }
    }

    fn nearpow2(n: i64) -> i64 {
        let mut x = n;
        x |= x >> 1;
        x |= x >> 2;
        x |= x >> 4;
        x |= x >> 8;
        x |= x >> 16;
        x |= x >> 32;
        x += 1;
        x >>= 1;
        x
    }

    fn polynomial_mul_reduction(&mut self, expr: &mut ir::Expr) {
        // polynomial distribution & addition algorithm
        
        if let ir::Expr::Binary { left, right, .. } = expr {
            if let ir::Expr::Literal(ir::Literal::Int(r)) = &**right {
                let target = *r;
                let np2 = Strength::nearpow2(target);
                println!("target: {}  --  Near pow2: {}", target, np2);
                *expr = ir::Expr::Binary {
                    op: ir::BinaryOp::Add,
                    left: Box::new(ir::Expr::Binary {
                        op: ir::BinaryOp::Shl,
                        left: Box::new(*left.clone()),
                        right: Box::new(ir::Expr::Literal(ir::Literal::Int(np2)))
                    }),
                    right: Box::new(ir::Expr::Binary {
                        op: ir::BinaryOp::Mul,
                        left: Box::new(*left.clone()),
                        right: Box::new(ir::Expr::Literal(ir::Literal::Int(target - np2)))
                    })
                }
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut ir::Expr) {
        // if binary op, check if we can simplify
        if let ir::Expr::Binary { op, left, right } = expr {

            if let ir::Expr::Literal(ir::Literal::Int(_)) = &**left {
                if let ir::Expr::Literal(_) = &**right {}
                else {
                    // switch the order
                    let temp = left.clone();
                    *left = right.clone();
                    *right = temp.clone();
                    self.changed = true;
                    return;
                }
            }
            else if let ir::Expr::Literal(ir::Literal::Int(r)) = &**right {
                if let ir::Expr::Literal(_) = &**left {}
                else {
                    match op {
                        ir::BinaryOp::Add => {
                            if *r == 0 {
                                *expr = *left.clone();
                                self.changed = true;
                                return;
                            }
                        }
                        ir::BinaryOp::Mul => {
                            if *r == 0 {
                                *expr = ir::Expr::Literal(ir::Literal::Int(0));
                                self.changed = true;
                                return;
                            }
                            else if *r == 1 {
                                *expr = *left.clone();
                                self.changed = true;
                                return;
                            } else if is_pow2(*r) {
                                *expr = ir::Expr::Binary {
                                    op: ir::BinaryOp::Shl,
                                    left: Box::new(*left.clone()),
                                    right: Box::new(ir::Expr::Literal(ir::Literal::Int(*r >> 1)))
                                };
                                self.changed = true;
                                return;
                            }
                            else {
                                self.polynomial_mul_reduction(expr);
                                self.changed = true;
                                return;
                            }
                        }
                        ir::BinaryOp::Div => {
                            if *r == 0 {
                                // Shouldn't happen as it will be checked on the compiler level
                                *expr = *left.clone();
                                self.changed = true;
                                return;
                            }
                            else if is_pow2(*r) {
                                *expr = ir::Expr::Binary {
                                    op: ir::BinaryOp::Shr,
                                    left: Box::new(*left.clone()),
                                    right: Box::new(ir::Expr::Literal(ir::Literal::Int(*r >> 1)))
                                };
                                self.changed = true;
                                return;
                            }
                        }
                        _ => {}
                    }
                }
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

impl opt::OptimizationPass for Strength {
    fn run(&mut self, ir: &mut ir::Module) -> bool {
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