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

    // Muestra la estructura de forma legible (un nivel de suma plano)
    pub fn mostrar(&self, indent: usize) {
        let prefijo = "  ".repeat(indent);
        match self {
            NodoArbol::Numero(n) => println!("{}Numero({})", prefijo, n),
            NodoArbol::Suma(hijos) => {
                println!("{}Suma [{} sumandos]", prefijo, hijos.len());
                for hijo in hijos {
                    hijo.mostrar(indent + 1);
                }
            }
        }
    }
}

pub fn main() {
    let mut arbol = NodoArbol::nuevo_numero(0);

    loop {
        println!("\n=== Gestor de Árbol de Sumas (n-ario) ===");
        println!("1. Nuevo número (reiniciar)");
        println!("2. Añadir sumando (mismo nodo Suma si ya existe)");
        println!("3. Ver estructura");
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
                    // Usa API n-aria: varios sumandos bajo un solo Suma
                    arbol = NodoArbol::agregar_sumando(arbol, nuevo_nodo);
                    println!("Sumando añadido al árbol n-ario.");
                }
            }
            "3" => {
                println!("\nEstructura actual:");
                arbol.mostrar(0);
            }
            "4" => break,
            _ => println!("Opción no válida"),
        }
    }
}
