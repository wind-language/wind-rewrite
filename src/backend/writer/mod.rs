
#[derive(Clone)]
pub struct Label {
    pub name: String,
    pub offset: usize,
    pub code: Vec<u8>,
}
impl Label {
    pub fn new(name: String, offset: usize) -> Label {
        Label {
            name,
            offset,
            code: Vec::new(),
        }
    }
    pub fn add_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }
    pub fn add_bytes(&mut self, bytes: Vec<u8>) {
        for byte in bytes {
            self.code.push(byte);
        }
    }
    pub fn add_bytes_at(&mut self, bytes: Vec<u8>, offset: usize) {
        for (i, byte) in bytes.iter().enumerate() {
            self.code[offset + i] = *byte;
        }
    }
}

#[derive(Clone)]
pub struct Section {
    pub name: String,
    pub labels: Vec<Label>,
    pub current_label: usize,
}
impl Section {
    pub fn new(name: String) -> Section {
        Section {
            name,
            labels: Vec::new(),
            current_label: 0,
        }
    }
}


#[derive(Clone)]
pub struct CodeBuilder {
    pub sections: Vec<Section>,
    pub current_section: usize,
}
impl CodeBuilder {
    pub fn new() -> CodeBuilder {
        CodeBuilder {
            sections: Vec::new(),
            current_section: 0,
        }
    }
    pub fn add_section(&mut self, name: String) -> usize {
        self.sections.push(Section::new(name));
        self.sections.len() - 1
    }
    pub fn get_section(&mut self, name: &str) -> Option<&mut Section> {
        for section in self.sections.iter_mut() {
            if section.name == name {
                return Some(section);
            }
        }
        None
    }
    pub fn bind_section(&mut self, i: usize) {
        self.current_section = i;
    }

    pub fn add_label(&mut self, name: String) -> usize {
        let section = &mut self.sections[self.current_section];

        let mut offset = 0;
        if section.labels.len() > 0 {
            offset = section.labels[section.labels.len() - 1].offset + section.labels[section.labels.len() - 1].code.len();
        }
        section.labels.push(Label::new(name, offset));
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
    pub fn add_bytes_at(&mut self, bytes: Vec<u8>, offset: usize) {
        let section = &mut self.sections[self.current_section];
        let label = &mut section.labels[section.current_label];
        label.add_bytes_at(bytes, offset);
    }
    
}