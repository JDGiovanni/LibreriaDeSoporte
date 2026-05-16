use std::fs;
use std::io::Write;

pub struct FileHelper;

impl FileHelper {
    // File Reader: Retorna un Smart Pointer (Box) para manejar el texto
    pub fn read_file(path: &str) -> Result<Box<str>, std::io::Error> {
        let content = fs::read_to_string(path)?;
        Ok(content.into_boxed_str())
    }

    // File Writer: Escribe datos en un archivo
    pub fn write_file(path: &str, content: &str) -> Result<(), std::io::Error> {
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}