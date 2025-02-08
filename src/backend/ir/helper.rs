pub mod mangling {
    use crate::backend::ir::DataType;
    use xxhash_rust::xxh3::xxh3_64;

    pub fn mangle(name: String, arguments: Vec<(String, DataType)>, return_type: DataType) -> String {
        let mut metadata = format!("{}(", name);
        for (i, (_, data_type)) in arguments.iter().enumerate() {
            metadata.push_str(&format!("{}", data_type));
            if i != arguments.len() - 1 {
                metadata.push_str(", ");
            }
        }
        metadata.push_str(&format!(")->{}", return_type));
        metadata
    }

    pub fn hash(metadata: String) -> String {
        format!("func_{:x}", xxh3_64(metadata.as_bytes()) as u64)
    }
}