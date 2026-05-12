use std::collections::HashMap;
use std::io;

pub fn main() {
    let mut notas = HashMap::new();
    notas.insert("Programacion Visual", 20);
    notas.insert("Diseño de Compiladores", 15);
    notas.insert("Sistemas Operativos", 12);

    println!("Materia a buscar: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    let input1 = input.trim();

    match notas.get(input1) {
        Some(nota) => println!("{} {}", input1, nota),
        None => println!("No existe esa materia"),
    }

}
