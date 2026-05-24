use std::io;
use std::process;

// Elida — mapa hecho en código (sin HashMap de std)
#[derive(Debug, Default)]
struct MapaNotas {
    // Guarda pares (materia, nota)
    datos: Vec<(String, i32)>,
}

impl MapaNotas {
    fn new() -> Self {
        Self { datos: Vec::new() }
    }

    // Inserta o actualiza nota por materia
    fn insert(&mut self, materia: String, nota: i32) {
        if let Some((_, n)) = self.datos.iter_mut().find(|(m, _)| *m == materia) {
            *n = nota;
        } else {
            self.datos.push((materia, nota));
        }
    }

    // Busca nota por materia
    fn get(&self, materia: &str) -> Option<i32> {
        self.datos
            .iter()
            .find(|(m, _)| m == materia)
            .map(|(_, n)| *n)
    }

    // Elimina materia y retorna true si existía
    fn remove(&mut self, materia: &str) -> bool {
        if let Some(pos) = self.datos.iter().position(|(m, _)| m == materia) {
            self.datos.remove(pos);
            true
        } else {
            false
        }
    }

    fn iter(&self) -> impl Iterator<Item = &(String, i32)> {
        self.datos.iter()
    }
}

pub fn main() {
    let mut notas = MapaNotas::new();
    notas.insert("Diseño de Compiladores".to_string(), 20);
    notas.insert("Programacion Visual".to_string(), 19);
    notas.insert("Sistemas Operativos".to_string(), 18);

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

                let nota: i32 = nota
                    .trim()
                    .parse()
                    .expect("Debe ser un número");

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

                if notas.remove(materia) {
                    println!("Materia eliminada");
                } else {
                    println!("No existe esa materia");
                }
            }
            "4" => {
                println!("\n===== NOTAS =====");
                for (materia, nota) in notas.iter() {
                    println!("{} -> {}", materia, nota);
                }
            }
            "5" => {
                process::exit(0);
            }
            _ => {
                println!("Opción inválida");
            }
        }
    }
}
