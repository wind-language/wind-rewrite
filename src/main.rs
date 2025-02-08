use wind::usr;

pub fn main() {
    if let Err(e) = usr::run_cli() {
        eprintln!("{}", e);
    }
}