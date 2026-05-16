use std::{collections::VecDeque, io};
use std::process;

pub struct Queue<T> {
    elements: VecDeque<T>
}

impl<T: std::fmt::Debug> Queue<T> {

    pub fn new() -> Self {
        Queue {
            elements: VecDeque::new() 
        }
    }
    pub fn push_back(&mut self, item: T) {
        self.elements.push_back(item);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn ver_cola (&self) {
        println!("----");
        for e in self.elements.iter() {
            print!("{:?} -> ", e);
        }
        println!("");
        println!("----");
    }
}
pub fn menu() {
    let mut cola = Queue::new();
    while true {
    println!("===Cola creada===\n
    1. Crear otra cola vacia.\n
    2. Añadir un valor a la cola.\n
    3. Eliminar el frente de la cola.\n
    4. Ver cola.\n
    5. Salir.");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Error");

    let opcion_numero: i32 = option
        .trim()
        .parse()
        .expect("No es un numero");

    match opcion_numero {
        1 => {
        cola = Queue::new();
        println!("Cola vacia creada.");
        }
        2 => {
        let mut input = String::new();
        println!("Ingresa el número a encolar:");
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();

        let numero: i32 = input
            .trim()
            .parse()
            .unwrap();
        
        cola.push_back(numero);
        }
        3 => {
            match cola.pop_front() {
                Some(valor) => println!("Elemento eliminado: {:?}", valor),
                None => println!("Cola vacia.")
            }
        }
        4 => {
        cola.ver_cola();
        }
        _ => process::exit(0),
        }
    }
}
pub fn main() {
    menu();
}