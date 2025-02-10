
#[cfg(test)]
mod tests {
    #[path = "print/tree_init.rs"]
    mod tree_init_test;

    #[path = "opts/const_folding.rs"]
    mod const_folding_test;
    
    #[path = "opts/dead_code.rs"]
    mod dead_code_test;
    
    #[path = "opts/strength.rs"]
    mod strength_test;
    
}