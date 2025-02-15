#[cfg(test)]
mod assemble_test {
    use wind::backend::assembler::x86_64::x86::{self, instructions::*};

    #[allow(unused_macros)]
    macro_rules! print_mc {
        ($($x:expr),*) => {
            println!("{}", $($x),*.iter().map(|x| format!("{:02X}", x)).collect::<Vec<String>>().join(" "));
        };
    }

    #[test]
    fn asm_assemble_mov() {
        assert_eq!((x86::RAX, x86::R15).mov(), vec![0x4C, 0x89, 0xF8]);
        assert_eq!((x86::R15D, 128).mov(), vec![0x41, 0xBF, 0x80, 0x00, 0x00, 0x00]);
        assert_eq!((x86::RAX, x86::ptr(x86::RBP, -256, 8)).mov(), vec![0x48, 0x8B, 0x85, 0x00, 0xFF, 0xFF, 0xFF]);
        assert_eq!((x86::ptr(x86::RBP, -128, 4), x86::ECX).mov(), vec![0x89, 0x4D, 0x80]);
        assert_eq!((x86::ptr(x86::RBP, -128, 2), 128).mov(), vec![0x66, 0xC7, 0x45, 0x80, 0x80, 0x00]);
    }

    #[test]
    fn asm_assemble_add() {
        assert_eq!((x86::RAX, x86::RCX).add(), vec![0x48, 0x01, 0xC8]);
        assert_eq!((x86::R15D, 128).add(), vec![0x41, 0x81, 0xC7, 0x80, 0x00, 0x00, 0x00]);
        assert_eq!((x86::RAX, x86::ptr(x86::RBP, -256, 8)).add(), vec![0x48, 0x03, 0x85, 0x00, 0xFF, 0xFF, 0xFF]);
        assert_eq!((x86::ptr(x86::RBP, -128, 4), x86::ECX).add(), vec![0x01, 0x4D, 0x80]);
        assert_eq!((x86::ptr(x86::RBP, -128, 2), 128).add(), vec![0x66, 0x81, 0x45, 0x80, 0x80, 0x00]);
    }

    #[test]
    fn asm_assemble_sub() {
        assert_eq!((x86::RAX, x86::RCX).sub(), vec![0x48, 0x2B, 0xC1]);
        assert_eq!((x86::R15D, 128).sub(), vec![0x41, 0x81, 0xEF, 0x80, 0x00, 0x00, 0x00]);
        assert_eq!((x86::RAX, x86::ptr(x86::RBP, -256, 8)).sub(), vec![0x48, 0x2B, 0x85, 0x00, 0xFF, 0xFF, 0xFF]);
        assert_eq!((x86::ptr(x86::RBP, -128, 4), x86::ECX).sub(), vec![0x29, 0x4D, 0x80]);
        assert_eq!((x86::ptr(x86::RBP, -128, 2), 128).sub(), vec![0x66, 0x81, 0x6D, 0x80, 0x80, 0x00]);
    }
}