use std::collections::HashMap;
use std::io;

pub fn main() {
    let mut notas = HashMap::new();

    loop {
        println!("\n== MENÚ ==");
        println!("1. Añadir nota");
        println!("2. Buscar nota");
        println!("3. Eliminar nota");
        println!("4. Mostrar todas");
        println!("5. Salir");

        let mut opcion = String::new();

        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer");

        let opcion = opcion.trim();

        match opcion {
            "1" => {
                let mut materia = String::new();
                let mut nota = String::new();

                println!("Nombre de la materia:");
                io::stdin()
                    .read_line(&mut materia)
                    .expect("Error");

                println!("Nota:");
                io::stdin()
                    .read_line(&mut nota)
                    .expect("Error");

                let nota: i32 = nota.trim().parse().expect("Debe ser un número");

                notas.insert(materia.trim().to_string(), nota);

                println!("Materia añadida correctamente");
            }

            "2" => {
                let mut materia = String::new();

                println!("Materia a buscar:");
                io::stdin()
                    .read_line(&mut materia)
                    .expect("Error");

                let materia = materia.trim();

                match notas.get(materia) {
                    Some(nota) => println!("{} -> {}", materia, nota),
                    None => println!("No existe esa materia"),
                }
            }

            "3" => {
                let mut materia = String::new();

                println!("Materia a eliminar:");
                io::stdin()
                    .read_line(&mut materia)
                    .expect("Error");

                let materia = materia.trim();

                match notas.remove(materia) {
                    Some(_) => println!("Materia eliminada"),
                    None => println!("No existe esa materia"),
                }
            }

            "4" => {
                println!("\n===== NOTAS =====");

                for (materia, nota) in &notas {
                    println!("{} -> {}", materia, nota);
                }
            }

            "5" => {
                println!("Saliendo...");
                break;
            }

            _ => {
                println!("Opción inválida");
            }
        }
    }
}
