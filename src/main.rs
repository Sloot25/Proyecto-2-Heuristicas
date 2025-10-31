mod grafica;
mod constructor_grafica;
mod pesoa;
mod svgs;


use std::collections::BTreeSet;
use std::fs::read_to_string;
use crate::grafica::Grafica; 
use crate::constructor_grafica::Constructor_grafica;
use crate::svgs::construir_svgs;
use crate::pesoa::PeSOA;
use std::collections::HashSet;
use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::fs;
use std::fmt::format;
use std::thread;


fn lanzar_pesoa_concurrente(num_pinguinos: usize, num_grupos: usize, niveles: usize, clavados: usize, epsilon: f64, grafica: Grafica, k:usize, semilla_inicial: i64, semilla_final: i64, int_to_id:HashMap<usize, String>, pesos_aristas: Vec<f64>, hilos: usize) {
    let semillas_q : VecDeque<i64> = (semilla_inicial..semilla_final).collect();
    let cola = Arc::new(Mutex::new(semillas_q));
    let mut handles = Vec::new();

    
    for _i in 0..hilos {
        let cola_2 = Arc::clone(&cola);
        let g = grafica.clone();
        let int_to_id_2 = int_to_id.clone();
        let pesos_aristas_c = pesos_aristas.clone();
        let handle = thread::spawn(move || {
            loop {
                let semilla_opt;
                {
                    let mut cola_block = cola_2.lock().unwrap();
                    semilla_opt = cola_block.pop_front();
                }
                if let Some(semilla) = semilla_opt {
                    lanzar_pesoa(num_pinguinos, num_grupos, niveles, clavados, epsilon, g.clone(), k, semilla, int_to_id_2.clone(), pesos_aristas_c.clone());
                } else {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


fn lanzar_pesoa(num_pinguinos: usize, num_grupos: usize, niveles: usize, clavados: usize, epsilon: f64, grafica: Grafica, k: usize, semilla: i64,int_to_id: HashMap<usize, String>, pesos_aristas: Vec<f64>) -> std::io::Result<()> {
    let mut heuristica = PeSOA::new(grafica, pesos_aristas, k, semilla);
    heuristica.completar_grafica();
    println!("N: {}", heuristica.normalizador);
    println!("D: {}", heuristica.diametro_grafica);
    heuristica.iniciar_PeSOA(num_pinguinos, num_grupos);
    heuristica.run_pesoa(niveles, clavados, epsilon);
    
    if let Some(mejor_solucion) = heuristica.mejor_pinguino_actual {
        println!("fitness: {} Semilla {}", mejor_solucion.fitness, semilla);
        let mut vector_nombres = Vec::new();
        for v in &mejor_solucion.solucion {
            vector_nombres.push(int_to_id.get(&v).unwrap());
        }
        //println!("Vertices {:?}", vector_nombres);

        let mut grafiquita = heuristica.grafica.arbol_generador_minimo(mejor_solucion.solucion.clone(), *mejor_solucion.solucion.iter().next().unwrap(), 40);
        //println!("{}", grafiquita.peso_arbol_generador);
        grafiquita.generarAdyacencias(int_to_id.clone());

        let ruta = format!("s{}_k{}_p{}_g{}_i{}_cl{}", semilla, k, num_pinguinos, num_grupos, niveles,clavados);

        let mut file = File::create(format!("resultados/{}.svg", ruta))?;
        let contenido = format!("Fitness {}\nSemilla {}\nVertices {:?}", mejor_solucion.fitness, semilla, vector_nombres);

        let _ = file.write_all(contenido.as_bytes())?;

        let _ = construir_svgs(grafiquita, ruta, int_to_id);
        // /grafiquita.bfs_ses(int_to_id);
        //println!("{}", grafiquita.to_string());
        
        //println!("{}", heuristica.grafica.to_string());
    } else {
        println!("No se encontró una solución.");
    }

    Ok(())
    
}


fn main() {
    let args: Vec<String> = env::args().collect();


    let clavados = 5;
    let epsilon = 0.0001;
    
    let k = args[4].parse::<usize>().expect("Error al parsear k");

    let num_pinguinos = 120;
    let num_grupos = 20;
    let niveles = 240;

    let semilla = args[2].parse::<i64>().expect("Error al parsear semilla");
    let semilla_2  = args[3].parse::<i64>().expect("Error al parsear semilla");
    let archivo = args[1].clone();
    let mut constructor = Constructor_grafica::new(archivo.clone());
    let mut g = constructor.cargar_datos();

    /*let archivo_2 = args[5].clone();

    let mut constructor = Constructor_grafica::new(archivo.clone());
    let mut g = constructor.cargar_datos();
    let mut set = BTreeSet::new();

    let contenido:String = read_to_string(archivo_2.clone()).unwrap();
    let lineas : Vec<&str> = contenido.lines().collect();
    for linea in lineas.iter() {
        println!("{}", linea);
        set.insert(*constructor.id_to_int.get(&linea.to_string()).unwrap());
    }
    println!("{}", g.arbol_generador_minimo(set.clone(), *set.iter().next().unwrap(), k).peso_arbol_generador);*/
    lanzar_pesoa_concurrente(num_pinguinos, num_grupos, niveles, clavados, epsilon, g.clone(), k, semilla, semilla_2, constructor.int_to_id, constructor.pesos_aristas, 6);

    
}

