use crate::backend::{ir, opt};

struct VisitState {
    func: Option<Box<ir::Function>>,
    can_die: bool,
    has_died: bool
}

pub struct DeadCode {
    changed: bool,
    state: VisitState,
}

impl DeadCode {
    pub fn new() -> Self {
        DeadCode {
            changed: false,
            state: VisitState {
                func: None,
                can_die: true, // Start with the assumption that code can die
                has_died: false
            }
        }
    }
    
    fn visit_expr(&mut self, expr: &mut ir::Expr) -> bool {
        match expr {
            ir::Expr::Binary { op, .. } => {
                // if it's a binary expression with no side effects, we can remove it
                match op {
                    _ => {
                        // can remove this expression as it has no side effects
                        return false;
                    } 
                }
            }
            ir::Expr::Literal(_) => {
                // literals have no side effects
                return false;
            }
            _ => {}
        }
        true
    }

    fn visit_stat(&mut self, stat: &mut ir::Statement) -> bool {
        match stat {
            ir::Statement::Return(_) => {
                if self.state.can_die {
                    self.state.has_died = true;
                }
                true
            }

            ir::Statement::Expr(expr) => {
                self.visit_expr(expr)
            }

            // TODO: Extend to conditional branches when we have them

            /* _ => {
                true
            } */
        }
    }
}

impl opt::OptimizationPass for DeadCode {

    fn run(&mut self, ir: &mut ir::Module) -> bool {
        for d_fn in ir.functions.iter_mut() {
            let mut new_body: Vec<ir::Statement> = Vec::new();
            self.state.func = Some(Box::new(d_fn.1.clone()));
            self.state.has_died = false; self.state.can_die = true;

            for stat in d_fn.1.body.iter_mut() {
                if self.visit_stat(stat) {
                    new_body.push(stat.clone())
                } else { self.changed = true; }
                if self.state.has_died {
                    self.changed = true;
                    break;
                }
            }

            if self.changed {
                d_fn.1.body = new_body;
            }
    
            self.state.func = None;
        }
        self.changed
    }
    
}