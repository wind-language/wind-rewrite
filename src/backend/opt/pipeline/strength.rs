use crate::backend::{ir, opt};

mod math_utils {
    pub fn compute_magic(d: u64, bits: u32) -> (u64, u32) {
        if d == 0 {
            panic!("Division by zero");
        }
        let d = d as u128;
        let w = bits;
    
        let two_to_w = 1u128 << w;
        let max_val = two_to_w - 1;
    
        let mut p = w - 1;
        let nc = (max_val - ((max_val - d + 1) % d)) + 1;
    
        let mut q1 = (1u128 << (w - 1)) / nc;
        let mut r1 = (1u128 << (w - 1)) - q1 * nc;
        let mut q2 = (((1u128 << (w - 1)) - 1)) / d;
        let mut r2 = (((1u128 << (w - 1)) - 1)) - q2 * d;
        let mut delta;
    
        loop {
            p += 1;
            if r1 >= nc - r1 {
                q1 = q1 * 2 + 1;
                r1 = r1 * 2 - nc;
            } else {
                q1 = q1 * 2;
                r1 = r1 * 2;
            }
            if r2 + 1 >= d - r2 {
                q2 = q2 * 2 + 1;
                r2 = r2 * 2 + 1 - d;
            } else {
                q2 = q2 * 2;
                r2 = r2 * 2 + 1;
            }
            delta = d - 1 - r2;
            if p >= (w * 2) || !(q1 < delta || (q1 == delta && r1 == 0)) {
                break;
            }
        }

        let magic = (q2 + 1) as u64;
        let shift = (p - w) as u32 + bits;
    
        (magic, shift)
    }
    
}

struct VisitState {
    func: Option<Box<ir::Function>>,
    no_opts: Vec<Box<ir::Expr>>
}

pub struct Strength {
    changed: bool,
    state: VisitState
}

fn is_pow2(n: u64) -> bool {
    n & (n - 1) == 0
}

impl Strength {
    pub fn new() -> Self {
        Strength {
            changed: false,
            state: VisitState {
                func: None,
                no_opts: vec![]
            }
        }
    }

    fn lwbd_pow2(n: u64) -> u32 {
        // lower bound power of 2
        63 - n.leading_zeros()
    }

    fn polynomial_mul_reduction(&mut self, expr: &mut ir::Expr) {
        // polynomial distribution & addition algorithm
        
        if let ir::Expr::Binary { left, right, .. } = expr {
            if let ir::Expr::Literal(ir::Literal::Int(r)) = &**right {
                let target = *r;
                let np2 = Strength::lwbd_pow2(target) as u64;
                *expr = ir::Expr::Binary {
                    op: ir::BinaryOp::Add,
                    left: Box::new(ir::Expr::Binary {
                        op: ir::BinaryOp::Shl,
                        left: Box::new(*left.clone()),
                        right: Box::new(ir::Expr::Literal(ir::Literal::Int(np2))),
                    }),
                    right: Box::new(ir::Expr::Binary {
                        op: ir::BinaryOp::Mul,
                        left: Box::new(*left.clone()),
                        right: Box::new(ir::Expr::Literal(ir::Literal::Int(target - (1 << np2)))),
                    }),
                }
            }
        }
    }

    fn magic_div_reduction(&mut self, expr: &mut ir::Expr, left_bits: u32) {
        // modular inverse multiplication algorithm
        
        if let ir::Expr::Binary { left, right, .. } = expr {
            if let ir::Expr::Literal(ir::Literal::Int(r)) = &**right {
                let magic = math_utils::compute_magic(*r, left_bits);
                *expr = ir::Expr::Binary {
                    op: ir::BinaryOp::Shr,
                    left: Box::new(ir::Expr::Binary {
                        op: ir::BinaryOp::Mul,
                        left: Box::new(*left.clone()),
                        right: Box::new( ir::Expr::Literal ( ir::Literal::Int( magic.0 as u64 ) ) ),
                    }),
                    right: Box::new( ir::Expr::Literal ( ir::Literal::Int( magic.1 as u64 ) ) ),
                };
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut ir::Expr) {
        // if binary op, check if we can simplify
        let expr_np = expr.clone();
        if let ir::Expr::Binary { op, left, right} = expr{
            if self.state.no_opts.contains(&Box::new(expr_np)) { return; }
            self.visit_expr(left);
            if let ir::Expr::Literal(ir::Literal::Int(_)) = &**left {
                if let ir::Expr::Literal(_) = &**right {}
                else {
                    // switch the order
                    /* let temp = left.clone();
                    *left = right.clone();
                    *right = temp.clone();
                    self.changed = true;
                    return; */
                }
            }
            else if let ir::Expr::Literal(ir::Literal::Int(r)) = &**right {
                let l_type = left.infer_type(None);
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
                                    right: Box::new(ir::Expr::Literal(ir::Literal::Int(*r >> 1))),
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
                                if let ir::DataType::Scalar { signed, .. } = l_type {
                                    if !signed {
                                        *expr = ir::Expr::Binary {
                                            op: ir::BinaryOp::Shr,
                                            left: Box::new(*left.clone()),
                                            right: Box::new(ir::Expr::Literal(ir::Literal::Int(*r >> 1))),
                                        };
                                        self.changed = true;
                                        return;
                                    }
                                }
                            }
                            else {
                                if let ir::DataType::Scalar { size, signed } = l_type {
                                    if !signed {
                                        self.magic_div_reduction(expr, (size*8) as u32);
                                        self.state.no_opts.push(Box::new(expr.clone()));
                                        self.changed = true;
                                        return;
                                    }
                                }
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