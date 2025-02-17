use super::*;
use object::write::{Object, SectionId, Symbol, SymbolId, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness};
use crate::reporter::obj::ObjectError;

pub struct ObjectBuilder<'a> {
    pub symbols: std::collections::HashMap<String, (SectionId, SymbolId)>, // (name, (section_id, symbol_id))
    pub relocs: Vec<(String, (usize, usize))>, // (name, (primitive section_id, offset))
    pub sections: Vec<SectionId>,
    pub raw_obj: Object<'a>
}
impl<'a> ObjectBuilder<'a> {
    pub fn new() -> Self {
        ObjectBuilder {
            symbols: std::collections::HashMap::new(),
            relocs: Vec::new(),
            sections: Vec::new(),
            raw_obj: Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little)
        }
    }

    pub fn add_section(&mut self, name: String, kind: SectionKind) -> SectionId {
        let section_id = self.raw_obj.add_section(vec![], name.into(), kind);
        self.sections.push(section_id);
        section_id
    }

    pub fn process_ext(&mut self, cb: &CodeBuilder) {
        for (sect, symbol) in cb.external.clone() {
            let s_id = self.raw_obj.add_symbol(Symbol{
                name: symbol.clone().into(),
                value: 0,
                size: 0,
                kind: object::SymbolKind::Unknown,
                scope: object::SymbolScope::Unknown,
                section: SymbolSection::Undefined,
                weak: false,
                flags: object::SymbolFlags::None
            });
            self.symbols.insert(symbol, (self.sections[sect], s_id));
        }
    }

    pub fn init_sects(&mut self, cb: &CodeBuilder) {
        for section in cb.sections.clone() {
            self.add_section(section.name, section.kind);
        }
    }

    pub fn populate_sects(&mut self, cb: &CodeBuilder) -> Result<(), ObjectError> {
        for section in cb.sections.clone() {
            let mut code: Vec<u8> = Vec::new();
            for label in section.labels.clone() {
                code.extend(label.code.clone());
                let mut visibility: object::SymbolScope = object::SymbolScope::Linkage;
                if let Some('.') = label.name.chars().nth(0) {
                    continue;
                }
                if label.global {
                    visibility = object::SymbolScope::Dynamic;
                }
                let s_id = self.raw_obj.add_symbol(Symbol {
                    name: label.name.clone().into(),
                    value: label.offset as u64,
                    size: label.code.len() as u64,
                    kind: object::SymbolKind::Text,
                    scope: visibility,
                    weak: label.weak,
                    section: SymbolSection::Section(self.sections[section.i]),
                    flags: object::SymbolFlags::None,
                });
                self.symbols.insert(label.name.clone(), (self.sections[section.i], s_id));
            }
            self.raw_obj.section_mut(self.sections[section.i]).set_data(code, 1);
        }

        Ok(())
    }

    pub fn solve_relocs(&mut self, cb: &CodeBuilder) -> Result<(), ObjectError> {
        for (sect, symbol, offset) in cb.relocs.clone() {
            let (_, s_id) = self.symbols.get(&symbol).ok_or(
                ObjectError::SymbolNotFound{symbol: symbol.clone()}
            )?;
            self.raw_obj.add_relocation(self.sections[sect], object::write::Relocation{
                offset: offset as u64,
                addend: -4, // hardcoded for a 4-byte displacement field
                symbol: *s_id,
                flags: object::write::RelocationFlags::Generic{
                    kind: object::write::RelocationKind::Relative,
                    encoding: object::write::RelocationEncoding::X86RipRelative,
                    size: 32
                }
            }).map_err(|_| ObjectError::RelocationFailed{symbol: symbol.clone()})?;
        }

        Ok(())
    }

    pub fn scan_cb(&mut self, cb: &CodeBuilder) -> Result<(), ObjectError> {
        self.init_sects(cb);
        self.process_ext(cb);
        self.populate_sects(cb)?;
        self.solve_relocs(cb)?;

        Ok(())
    }

    pub fn write_obj(&mut self, path: &str) -> Result<(), ObjectError> {
        let buffer = self.raw_obj.write().map_err(|_| ObjectError::Unknown)?;
        std::fs::write(path, buffer).map_err(|_| ObjectError::FileWriteFailed{file: path.into()})?;

        Ok(())
    }
}