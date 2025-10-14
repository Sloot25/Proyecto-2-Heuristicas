mod grafica;
mod constructor_grafica;
mod gio;

use crate::grafica::Grafica; 
use crate::constructor_grafica::Constructor_grafica;
use crate::gio::GIO;
use std::collections::HashSet;


fn main() {
    let mut constructor = Constructor_grafica::new("archivo.txt".to_string());
    let mut g = constructor.cargar_datos();
    println!("{}",g.to_string());
    let mut a = HashSet::<usize>::new();
    for c in 0..g.size {
        a.insert(c);
    }

    
    println!("{}", g.arbol_generador_minimo(a, 0,5).to_string());
    let mut heuristica = GIO::new(g, constructor.pesos_aristas, 5);
    heuristica.completar_grafica();
}
