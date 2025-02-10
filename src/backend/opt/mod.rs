use crate::backend::ir;

const MAX_ITERATIONS: usize = 10;

pub trait OptimizationPass {
    /// Apply the pass to the IR. Returns `true` if the IR was modified.
    fn run(&mut self, ir: &mut ir::Module) -> bool;
}


pub mod pipeline {
    pub mod folding;
    pub mod dead_code;
    pub mod strength;
    //TODO: pub mod constant_propagation;
    //TODO: pub mod loop_unrolling;
    //TODO: pub mod cse;  (Maybe, pretty complex)
}

pub struct PassManager {
    passes: Vec<Box<dyn OptimizationPass>>,
    max_iterations: usize,
}

impl PassManager {
    pub fn new() -> Self {
        PassManager {
            passes: Vec::new(),
            max_iterations: MAX_ITERATIONS,
        }
    }

    pub fn add_pass<P: OptimizationPass + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }

    /// Run all passes until a fixed point is reached or a maximum number of iterations.
    pub fn run_all(&mut self, ir: &mut ir::Module) {
        for _ in 0..self.max_iterations {
            let mut changed = false;
            for pass in self.passes.iter_mut() {
                changed |= pass.run(ir);
            }
            if !changed {
                break;
            }
        }
    }
}