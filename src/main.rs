mod grafica;
mod constructor_grafica;
mod pesoa;

use crate::grafica::Grafica; 
use crate::constructor_grafica::Constructor_grafica;
use crate::pesoa::PeSOA;
use std::collections::HashSet;
use std::env;
use std::collections::HashMap;

fn lanzar_pesoa(num_pinguinos: usize, num_grupos: usize, niveles: usize, clavados: usize, epsilon: f64, grafica: Grafica, k: usize, semilla: i64,int_to_id: HashMap<usize, String>, pesos_aristas: Vec<f64>) {
    let mut heuristica = PeSOA::new(grafica, pesos_aristas, k, semilla);
    heuristica.completar_grafica();

    heuristica.iniciar_PeSOA(num_pinguinos, num_grupos);
    heuristica.run_pesoa(niveles, clavados, epsilon);

    if let Some(mejor_solucion) = heuristica.mejor_pinguino_actual {
        println!("Mejor fitness encontrado: {}", mejor_solucion.fitness);
        let mut vector_nombres = Vec::new();
        for v in &mejor_solucion.solucion {
            vector_nombres.push(int_to_id.get(&v).unwrap());
        }
        println!("Vertices {:?}", vector_nombres);
        // Opcional: Mapear los índices de la solución a los IDs originales de las ciudades/nodos
        let mut grafiquita = heuristica.grafica.arbol_generador_minimo(mejor_solucion.solucion.clone(), *mejor_solucion.solucion.iter().next().unwrap(), 40);
        println!("{}", grafiquita.peso_arbol_generador);
        grafiquita.generarAdyacencias(int_to_id);
        //println!("{}", grafiquita.to_string());
        //println!("{}", heuristica.grafica.to_string());
    } else {
        println!("No se encontró una solución.");
    }
    
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let num_pinguinos = 100;
    let num_grupos = 5;
    let niveles = 50;
    let clavados = 2;
    let epsilon = 0.001;
    
    let k = args[3].parse::<usize>().expect("Error al parsear k");
    let semilla = args[2].parse::<i64>().expect("Error al parsear semilla");
    let archivo = args[1].clone();

    let mut constructor = Constructor_grafica::new(archivo);
    let mut g = constructor.cargar_datos();
    
    lanzar_pesoa(num_pinguinos, num_grupos, niveles, clavados, epsilon, g.clone(), k, semilla, constructor.int_to_id, constructor.pesos_aristas);

    
}
