use std::io;
use std::process;
#[derive(Debug)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T: std::fmt::Debug> Stack<T> {

    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn ver_pila(&self) {
        println!("----");
        for e in self.elements.iter().rev() {
            println!("{:?}", e);
        }
        println!("----");
    }
}

pub fn menu() {
    let mut pila = Stack::new();
    loop {
    println!("=== Pila creada ===\n
        1. Añadir valor a la pila\n
        2. Sacar valor de la pila\n
        3. Ver tope\n
        4. Mostrar toda la pila\n
        5. Salir");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Error");
        let opcion_numero: i32 = option
            .trim()
            .parse()
            .unwrap_or(0);

        match opcion_numero {
            1 => {
                println!("Ingresa el número para la pila:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .unwrap();
                if let Ok(n) = input.trim().parse::<i32>() {
                    pila.push(n);
                    println!("{} agregado a la pila.", n);
                }
            }
            2 => {
                match pila.pop() {
                    Some(valor) => println!("Se sacó el valor: {:?}", valor),
                    None => println!("La pila está vacía."),
                }
            }
            3 => {
                match pila.peek() {
                    Some(valor) => println!("El tope es: {:?}", valor),
                    None => println!("La pila está vacía."),
                }
            }
            4 => {
                println!("Contenido de la pila (de arriba a abajo):");
                pila.ver_pila();
            }
            5 => process::exit(0),
            _ => println!("Opción no válida"),
        }
    }
}
pub fn main() {
    menu();
}
