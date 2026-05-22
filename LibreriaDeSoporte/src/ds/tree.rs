// Elida — árbol de sumas (estructura n-aria)

use std::io;

#[derive(Debug)]
pub enum NodoArbol {
    Numero(i32),
    Cadena(String),
    Token(String),
    // N sumandos en un solo nodo (ya no izq/der fijos)
    Suma(Vec<Box<NodoArbol>>),
}

impl NodoArbol {
    pub fn nuevo_numero(valor: i32) -> Box<Self> {
        Box::new(NodoArbol::Numero(valor))
    }

    pub fn nueva_cadena(valor: String) -> Box<Self> {
        Box::new(NodoArbol::Cadena(valor))
    }

    pub fn nuevo_token(valor: String) -> Box<Self> {
        Box::new(NodoArbol::Token(valor))
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
            NodoArbol::Cadena(texto) => println!("{}Cadena(\"{}\")", prefijo, texto),
            NodoArbol::Token(token) => println!("{}Token({})", prefijo, token),
            NodoArbol::Suma(hijos) => {
                println!("{}Suma [{} sumandos]", prefijo, hijos.len());
                for hijo in hijos {
                    hijo.mostrar(indent + 1);
                }
            }
        }
    }
}

fn leer_nodo_desde_entrada(prompt: &str) -> Option<Box<NodoArbol>> {
    println!("{}", prompt);
    println!("Formato: num:10 | str:hola | tok:IDENT");
    let mut val = String::new();
    io::stdin().read_line(&mut val).ok()?;
    let entrada = val.trim();

    if let Some(num) = entrada.strip_prefix("num:") {
        if let Ok(n) = num.trim().parse::<i32>() {
            return Some(NodoArbol::nuevo_numero(n));
        }
        println!("Número inválido.");
        return None;
    }

    if let Some(texto) = entrada.strip_prefix("str:") {
        return Some(NodoArbol::nueva_cadena(texto.trim().to_string()));
    }

    if let Some(token) = entrada.strip_prefix("tok:") {
        return Some(NodoArbol::nuevo_token(token.trim().to_string()));
    }

    println!("Entrada inválida. Usa num:, str: o tok:");
    None
}

pub fn main() {
    let mut arbol = NodoArbol::nuevo_numero(0);

    loop {
        println!("\n=== Gestor de Árbol de Sumas (n-ario) ===");
        println!("1. Nuevo nodo base (reiniciar)");
        println!("2. Añadir nodo (num, str o tok)");
        println!("3. Ver estructura");
        println!("4. Salir");

        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .unwrap();
        let opcion = opcion.trim();

        match opcion {
            "1" => {
                if let Some(nodo) = leer_nodo_desde_entrada("Introduce el nodo base:") {
                    arbol = nodo;
                    println!("Árbol reiniciado.");
                }
            }
            "2" => {
                if let Some(nuevo_nodo) = leer_nodo_desde_entrada("Introduce el nodo a añadir:") {
                    // Usa API n-aria: varios sumandos bajo un solo Suma
                    arbol = NodoArbol::agregar_sumando(arbol, nuevo_nodo);
                    println!("Nodo añadido al árbol n-ario.");
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
