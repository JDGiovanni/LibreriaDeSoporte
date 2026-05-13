mod helpers;
use helpers::logger::{Logger, LogLevel};
use helpers::file_io::FileHelper;

fn main() {
    Logger::log(LogLevel::Info, "Iniciando pruebas de helpers...");
    Logger::log(LogLevel::Warning, "Esto es un aviso.");
    
    let test_content = "Datos de prueba para la libreria";
    if let Ok(_) = FileHelper::write_file("prueba.txt", test_content) {
        Logger::log(LogLevel::Info, "Archivo escrito correctamente.");
    }

    match FileHelper::read_file("prueba.txt") {
        Ok(data) => println!("Contenido leido (Smart Pointer): {}", data),
        Err(_) => Logger::log(LogLevel::Error, "No se pudo leer el archivo."),
    }
}