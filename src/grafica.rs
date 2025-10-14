use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;

use pheap::PairingHeap;

pub struct Vertice {
    elemento:String,
    distancia:i64,
    vecinos: HashMap<String, i64>,
}

pub struct Grafica {
    pub vertices: HashMap<String, Vertice>,
    pub peso_arbol_generador: f64,
    pub num_vertices: i64,
}

impl Grafica {

    pub fn new(archivo: String) -> Self {
        let mut g = Grafica{
            vertices: HashMap::new(),
            peso_arbol_generador: 0.0,
            num_vertices:0
        };
        g.cargar_datos(archivo);
        return g;
    }

    fn cargar_datos(&mut self, archivo: String) {
        let contenido:String = read_to_string(archivo).unwrap();
        let lineas: Vec<&str> = contenido
            .lines()
            .collect();

        for linea in lineas.iter() {
            let datos: Vec<&str> = linea.split(",").collect();
            let origen = datos[0].to_string();
            let destino = datos[1].to_string();
            self.agregar_vertice(origen.clone());
            self.agregar_vertice(destino.clone());
            self.agregar_arista(origen, destino, datos[2].parse::<i64>().unwrap());
        }

    }

    pub fn get_peso(&mut self, vertice: String, vecino: String ) -> Option<i64> {
        
        return  vertices.get(vertice).unwrap().vecinos.get(vecino);
        
    }

    pub fn to_string(&mut self) -> String {
        let mut resultado = String::new();
        let mut vertices_ordenados: Vec<_> = self.vertices.keys().collect();
        vertices_ordenados.sort();
        for nombre_vertice in vertices_ordenados {
            
            if let Some(vertice) = self.vertices.get(nombre_vertice) {
                writeln!(&mut resultado, "Vertice '{}':", vertice.elemento).unwrap();

                if vertice.vecinos.is_empty() {
                    writeln!(&mut resultado, "  (Sin vecinos)").unwrap();
                } else {
            
                    let mut vecinos_ordenados: Vec<_> = vertice.vecinos.keys().collect();
                    vecinos_ordenados.sort();
                    
                    for nombre_vecino in vecinos_ordenados {
                         if let Some(peso) = vertice.vecinos.get(nombre_vecino) {
                            writeln!(&mut resultado, "  -> '{}' (peso: {})", nombre_vecino, peso).unwrap();
                         }
                    }
                }
            }
        }
        resultado
    }

    fn agregar_vertice(&mut self, a:String) {

        self.vertices.entry(a.clone()).or_insert_with(|| Vertice{
            elemento: a,
            distancia:0,
            vecinos: HashMap::new(),
        });
        self.num_vertices +=1;
    }

    fn agregar_arista(&mut self, a:String, b: String, p:i64) {
        if let Some(vertice_a) = self.vertices.get_mut(&a){
            vertice_a.vecinos.insert(b.clone(),p);
        }
        if let Some(vertice_b) = self.vertices.get_mut(&b){
            vertice_b.vecinos.insert(a.clone(), p);
        }
    }
/// Recibe los vertices que forman parte del arbol para aristas 
    pub fn arbol_generador_minimo(&mut self, vertices:HashSet<String>, vertice_inicial: String) ->Grafica {
        let mut arbol = Grafica { vertices: HashMap::new(), peso_arbol_generador:0.0 };
        let mut heap = PairingHeap::<String, i64>::new();
        
        let mut visitados = HashSet::<String>::new();
        let mut peso_conexion_actual =  HashMap::<String, i64>::new();
        let mut padres = HashMap::<String, String>::new();

        for v in vertices.iter() {
            peso_conexion_actual.insert(v.clone(), i64::MAX);
        }
        
        peso_conexion_actual.insert(vertice_inicial.clone(), 0);
        heap.insert( vertice_inicial.clone(), 0);

        while let Some((vertice, costo)) = heap.delete_min() {
            if visitados.contains(&vertice) || !vertices.contains(&vertice) {
                continue;
            }

            visitados.insert(vertice.clone());

            arbol.agregar_vertice(vertice.clone());

            if let Some(padre) = padres.get(&vertice) {
                if let Some(peso) = self.vertices.get(padre).unwrap().vecinos.get(&vertice) {
                    arbol.agregar_arista(padre.clone(), vertice.clone(), *peso);
                    arbol.peso_arbol_generador += *peso as f64;
                }
            }

            if let Some(v_actual) = self.vertices.get(&vertice) {
                for (u, &peso_arista) in v_actual.vecinos.iter() {
                    if !visitados.contains(u) && peso_arista < *peso_conexion_actual.get(u).unwrap_or(&i64::MAX)  {
                        peso_conexion_actual.insert(u.clone(), peso_arista);
                        padres.insert(u.clone(), vertice.clone());
                        heap.insert(u.clone(),peso_arista);
                    }
                }

            }
            
        }

        arbol
    }
        
}
