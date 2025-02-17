use crate::backend::assembler::x86_64::instr::jmp::Jmp;
use crate::backend::assembler::x86_64::instr::call::Call;
use crate::reporter::asm::AssemblerError;

pub mod obj;

use object::SectionKind;

#[derive(Debug, Clone)]
pub struct Label {
    pub name: String,
    pub offset: isize,
    pub code: Vec<u8>,
    pub i: usize,
    pub global: bool,
    pub weak: bool,
}
impl Label {
    pub fn new(name: String, i: usize, offset: isize) -> Label {
        Label {
            name,
            offset,
            code: Vec::new(),
            i,
            global: false,
            weak: false
        }
    }
    pub fn set_global(&mut self) {
        self.global = true;
    }
    pub fn set_weak(&mut self) {
        self.weak = true;
    }
    pub fn add_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }
    pub fn add_bytes(&mut self, bytes: Vec<u8>) {
        for byte in bytes {
            self.code.push(byte);
        }
    }
    pub fn set_bytes_at(&mut self, bytes: Vec<u8>, offset: usize) {
        // need this for resolving jumps and such
        for (i, byte) in bytes.iter().enumerate() {
            self.code[offset + i] = *byte;
        }
    }
    pub fn resolve_rel_jmp(&mut self, offset: isize) {
        let addr_rel = offset - self.offset - (self.code.len() as isize);
        self.add_bytes(
            (addr_rel as isize).jmp()
        );
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub labels: Vec<Label>,
    current_label: usize,
    pub i: usize,
    pub kind: SectionKind
}
impl Section {
    pub fn new(name: String, i: usize) -> Section {
        Section {
            name,
            labels: Vec::new(),
            current_label: 0,
            i,
            kind: SectionKind::Text
        }
    }
}


#[derive(Debug, Clone)]
pub struct CodeBuilder {
    pub sections: Vec<Section>,
    current_section: usize,
    unresolved_jmps: Vec<(String, (Section, Label, isize))>, // (label_name, (section, label, offset))
    pub external: Vec<(usize, String)>, // (section, symbol)
    pub relocs: Vec<(usize, String, isize)> // (section, symbol, offset)
}
impl CodeBuilder {
    pub fn new() -> CodeBuilder {
        CodeBuilder {
            sections: Vec::new(),
            current_section: 0,
            unresolved_jmps: Vec::new(),
            external: Vec::new(),
            relocs: Vec::new()
        }
    }
    pub fn add_section(&mut self, name: String) -> usize {
        self.sections.push(Section::new(name, self.sections.len()));
        self.sections.len() - 1
    }
    pub fn set_kind(&mut self, kind: SectionKind) {
        self.sections[self.current_section].kind = kind;
    }
    pub fn get_section(&mut self, name: &str) -> Option<&mut Section> {
        for section in self.sections.iter_mut() {
            if section.name == name {
                return Some(section);
            }
        }
        None
    }
    pub fn add_extern(&mut self, symbol: String) {
        self.external.push(
            (self.current_section, symbol)
        );
    }
    pub fn bind_section(&mut self, i: usize) {
        self.current_section = i;
    }

    pub fn add_label(&mut self, name: String) -> usize {
        let section = &mut self.sections[self.current_section];

        let mut offset = 0;
        if section.labels.len() > 0 {
            offset = section.labels[section.labels.len() - 1].offset + section.labels[section.labels.len() - 1].code.len() as isize;
        }
        section.labels.push(Label::new(name, section.labels.len(), offset));
        section.labels.len() - 1
    }
    pub fn get_label(&mut self, name: &str) -> Option<&mut Label> {
        let section = &mut self.sections[self.current_section];
        for label in section.labels.iter_mut() {
            if label.name == name {
                return Some(label);
            }
        }
        None
    }
    pub fn bind_label(&mut self, i: usize) {
        let section = &mut self.sections[self.current_section];
        section.current_label = i;
    }
    pub fn set_global(&mut self) {
        let section = &mut self.sections[self.current_section];
        let label = &mut section.labels[section.current_label];
        label.set_global();
    }
    
    pub fn add_byte(&mut self, byte: u8) {
        let section = &mut self.sections[self.current_section];
        let label = &mut section.labels[section.current_label];
        label.add_byte(byte);
    }
    pub fn add_bytes(&mut self, bytes: Vec<u8>) {
        let section = &mut self.sections[self.current_section];
        let label = &mut section.labels[section.current_label];
        label.add_bytes(bytes);
    }
    pub fn set_bytes_at(&mut self, bytes: Vec<u8>, offset: isize) {
        let section = &mut self.sections[self.current_section];
        let label = &mut section.labels[section.current_label];
        label.set_bytes_at(bytes, offset as usize);
    }
    pub fn resolve_rel_jmp(&mut self, offset: isize) {
        let section = &mut self.sections[self.current_section];
        let label = &mut section.labels[section.current_label];
        label.resolve_rel_jmp(offset);
    }
    pub fn symbol_jmp(&mut self, name: String) {
        let section_clone = self.sections[self.current_section].clone();
        let label_clone = section_clone.labels[section_clone.current_label].clone();
        let section = &mut self.sections[self.current_section];
        let label = &mut section.labels[section.current_label];
        self.unresolved_jmps.push((name, (section_clone, label_clone, label.code.len() as isize)));
        let placeholder: Vec<u8> = (0x00 as isize).jmp();
        let current_section = self.current_section;
        let current_label = self.sections[current_section].current_label;
        self.sections[current_section].labels[current_label].add_bytes(placeholder);
    }
    pub fn symbol_call(&mut self, name:String) {
        let section_clone = self.sections[self.current_section].clone();
        let label_clone = section_clone.labels[section_clone.current_label].clone();
        let section = &mut self.sections[self.current_section];
        let label = &mut section.labels[section.current_label];
        self.unresolved_jmps.push((name, (section_clone, label_clone, label.code.len() as isize)));
        let placeholder: Vec<u8> = (0x00 as isize).call();
        let current_section = self.current_section;
        let current_label = self.sections[current_section].current_label;
        self.sections[current_section].labels[current_label].add_bytes(placeholder);
    }
    pub fn resolve_jmps(&mut self) -> Result<(), AssemblerError> {
        let mut sections = self.sections.clone();
        let unresolved_jmps = self.unresolved_jmps.clone();
        for (name, (src_section, src_label, offset)) in unresolved_jmps.iter() {
            let mut found = false;
            for section in sections.iter_mut() {
                for label in section.labels.iter_mut() {
                    if label.name == *name {
                        self.bind_section(src_section.i);
                        self.bind_label(src_label.i);
                        self.set_bytes_at(
                            (label.offset - src_label.offset - offset.clone() as isize).jmp(),
                            offset.clone()
                        );
                        found = true;
                        break;
                    }
                }
                if found { break; }
            }
            if !found {
                self.relocs.push((src_section.i, name.clone(), offset.clone()+src_label.offset+1));
            }
        }
        Ok(())
    }

    pub fn finalize(&mut self) -> Result<(), AssemblerError> {
        self.resolve_jmps()?;
        Ok(())
    }

    pub fn write_obj(&mut self, path: &str) {
        let mut obj = obj::ObjectBuilder::new();
        obj.scan_cb(self).unwrap();
        obj.write_obj(path).unwrap();
    }
}