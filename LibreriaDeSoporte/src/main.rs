mod helpers;
use helpers::logger::{Logger, LogLevel};
use helpers::file_io::FileHelper;

fn main() {
    let logger = Logger::new();

    logger.log(LogLevel::Info, "Iniciando pruebas de helpers...");
    logger.log(LogLevel::Warning, "Esto es un aviso.");
    
    let test_content = "Datos de prueba para la libreria";
    if let Ok(_) = FileHelper::write_file("prueba.txt", test_content) {
        logger.log(LogLevel::Info, "Archivo escrito correctamente.");
    }

    match FileHelper::read_file("prueba.txt") {
        Ok(data) => println!("Contenido leido: {}", data),
        Err(_) => logger.log(LogLevel::Error, "No se pudo leer el archivo."),
    }
}
