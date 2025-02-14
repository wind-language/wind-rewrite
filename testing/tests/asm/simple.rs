#[cfg(test)]
mod assemble_test {
    use wind::backend::assembler::x86_64::x86::{self, instructions::*};

    macro_rules! print_mc {
        ($($x:expr),*) => {
            println!("{}", $($x),*.iter().map(|x| format!("{:02X}", x)).collect::<Vec<String>>().join(" "));
        };
    }

    #[test]
    fn asm_assemble_mov() {
        print_mc!((x86::RAX, x86::R15).mov());
        print_mc!((x86::R15D, 128).mov());
        print_mc!((x86::RAX, x86::ptr(x86::RBP, -256)).mov());
    }
}