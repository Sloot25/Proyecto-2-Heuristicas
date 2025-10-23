mod grafica;
mod constructor_grafica;
mod pesoa;

use crate::grafica::Grafica; 
use crate::constructor_grafica::Constructor_grafica;
use crate::pesoa::PeSOA;
use std::collections::HashSet;


fn main() {
    let mut constructor = Constructor_grafica::new("archivo5.txt".to_string());
    let mut g = constructor.cargar_datos();
    //println!("{}",g.to_string());
    let mut a = HashSet::<usize>::new();
    for c in 0..g.size {
        a.insert(c);
    }

    let k = 4;
    //println!("{}", g.arbol_generador_minimo(a, 0,5).to_string());
    let mut heuristica = PeSOA::new(g, constructor.pesos_aristas, 4, 0);
    heuristica.completar_grafica();

    let num_pinguinos = 10;
    let num_grupos = 5;
    let max_iteraciones = 50;

    heuristica.iniciar_PeSOA(num_pinguinos, num_grupos);
    heuristica.run_pesoa(max_iteraciones);

    if let Some(mejor_solucion) = heuristica.mejor_pinguino_actual {
        println!("Mejor fitness encontrado: {}", mejor_solucion.fitness);
        // Opcional: Mapear los índices de la solución a los IDs originales de las ciudades/nodos
        let mut grafiquita = heuristica.grafica.arbol_generador_minimo(mejor_solucion.solucion.clone(), *mejor_solucion.solucion.iter().next().unwrap(), 4);
        println!("{}", grafiquita.peso_arbol_generador);
        //println!("{}", grafiquita.to_string());
        //println!("{}", heuristica.grafica.to_string());
    } else {
        println!("No se encontró una solución.");
    }
    
}
