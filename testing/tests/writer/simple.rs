#[cfg(test)]
mod writer_test {
    use wind::backend::{writer, assembler::x86_64::x86::{self, instructions::*}};
    
    #[allow(unused_macros)]
    macro_rules! print_mc {
        ($($x:expr),*) => {
            println!("{}", $($x),*.iter().map(|x| format!("{:02X}", x)).collect::<Vec<String>>().join(" "));
        };
    }

    #[test]
    #[ignore]
    fn wt_test_jmp() {
        let mut cb = writer::CodeBuilder::new();
        let section = cb.add_section("text".to_string());
        cb.bind_section(section);

        let label = cb.add_label("main".to_string());
        cb.bind_label(label);

        cb.add_bytes(
            (x86::RSP, 16).sub()
        );
        cb.add_bytes(
            (x86::RBP, x86::RSP).mov()
        );
        cb.add_bytes(
            (x86::ptr(x86::RBP, -8, 4), 0x10).mov()
        );
        cb.add_bytes(
            (x86::EAX, x86::ptr(x86::RBP, -8, 4)).mov()
        );

        let cl1 = cb.add_label(".L1".to_string());
        cb.bind_label(cl1);

        cb.add_bytes(
            (x86::ptr(x86::RBP, -8, 4), 0x20).add()
        );
        cb.symbol_jmp("main".to_string());

        cb.finalize().unwrap();
        cb.write_obj("output.elf");
    }

    #[test]
    fn wt_test_reloc_libc() {
        let mut cb = writer::CodeBuilder::new();
        let section = cb.add_section(".rodata".to_string());
        cb.bind_section(section);
        let str_label = cb.add_label(".str1".to_string());
        cb.bind_label(str_label);
        cb.add_bytes(
            "Hello, world!\n\0".as_bytes().to_vec()
        );

        let section = cb.add_section(".text".to_string());
        cb.bind_section(section);

        cb.add_extern("printf".to_string());
        cb.add_extern("exit".to_string());

        let label = cb.add_label("_start".to_string());
        cb.bind_label(label);
        cb.set_global();

        cb.add_bytes(
            (x86::RSI, x86::ptr(x86::RIP, 0x100, 8)).lea()
        );
        cb.add_bytes(
            (x86::EDI, 0x01).mov()
        );
        cb.symbol_call("exit".to_string());

        cb.finalize().unwrap();
        cb.write_obj("output.elf");
    }
}