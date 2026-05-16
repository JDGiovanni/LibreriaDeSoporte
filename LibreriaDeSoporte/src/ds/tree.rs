// Elida — árbol de sumas (estructura n-aria)

use std::io;

#[derive(Debug)]
pub enum NodoArbol {
    Numero(i32),
    // N sumandos en un solo nodo (ya no izq/der fijos)
    Suma(Vec<Box<NodoArbol>>),
}

impl NodoArbol {
    pub fn nuevo_numero(valor: i32) -> Box<Self> {
        Box::new(NodoArbol::Numero(valor))
    }

    // Crea suma con uno o más hijos
    pub fn nueva_suma_naria(hijos: Vec<Box<NodoArbol>>) -> Box<Self> {
        assert!(!hijos.is_empty(), "una suma necesita al menos un hijo");
        Box::new(NodoArbol::Suma(hijos))
    }

    // Compatibilidad binaria; el menú migrará a agregar_sumando en 3/3
    pub fn nueva_suma(izq: Box<NodoArbol>, der: Box<NodoArbol>) -> Box<Self> {
        Self::nueva_suma_naria(vec![izq, der])
    }

    // Añade un sumando: extiende el nodo Suma raíz o crea uno nuevo
    pub fn agregar_sumando(arbol: Box<Self>, sumando: Box<Self>) -> Box<Self> {
        match *arbol {
            NodoArbol::Suma(mut hijos) => {
                hijos.push(sumando);
                Box::new(NodoArbol::Suma(hijos))
            }
            _ => Box::new(NodoArbol::Suma(vec![arbol, sumando])),
        }
    }
}

pub fn main() {
    let mut arbol = NodoArbol::nuevo_numero(0);

    loop {
        println!("\n===Gestor de Árbol de Sumas ===");
        println!("1. Nuevo Número (Reiniciar)");
        println!("2. Sumar al Árbol (Árbol Actual + Nuevo Número)");
        println!("3. Ver Estructura");
        println!("4. Salir");

        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .unwrap();
        let opcion = opcion.trim();

        match opcion {
            "1" => {
                println!("Introduce el nuevo número:");
                let mut val = String::new();
                io::stdin()
                    .read_line(&mut val)
                    .unwrap();
                if let Ok(n) = val.trim().parse::<i32>() {
                    arbol = NodoArbol::nuevo_numero(n);
                    println!("Árbol reiniciado con {}", n);
                }
            }
            "2" => {
                println!("Introduce el número para sumar:");
                let mut val = String::new();
                io::stdin()
                    .read_line(&mut val)
                    .unwrap();

                if let Ok(n) = val.trim().parse::<i32>() {
                    let nuevo_nodo = NodoArbol::nuevo_numero(n);
                    arbol = NodoArbol::nueva_suma(arbol, nuevo_nodo);
                    println!("Suma añadida.");
                }
            }
            "3" => {
                println!("\nEstructura actual:");
                println!("{:?}", arbol);
            }
            "4" => break,
            _ => println!("Opción no válida"),
        }
    }
}
