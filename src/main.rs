mod grafica;

use crate::grafica::Grafica; 
use std::collections::HashSet;

fn main() {
    let mut g = Grafica::new("archivo.txt".to_string());
    
    println!("{}",g.to_string());
    let mut a = HashSet::<String>::new();
    let mut inicial = "A";
    for (nombre,_) in g.vertices.iter() {
        a.insert(nombre.to_string());
    }

    
    println!("{}", g.arbol_generador_minimo(a, inicial.to_string()).to_string())
}
