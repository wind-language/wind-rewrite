#![feature(portable_simd)]

pub mod usr;
pub mod reporter;
pub mod frontend;

fn main() {
    if let Err(e) = usr::run_cli() {
        eprintln!("{}", e);
    }
}
