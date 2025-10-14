use crate::Grafica;

pub GIO {
    pub solucion_actual: HashSet<String>,
    pub peso_solucion_actual: HashSet<String>,
    pub promedios: HashMap<String, f64>,
    grafica: Grafica,
}

// k-1 aristas existentes mas pesadas sumadas antes de completarla es el normalizador
// arbol generador de peso minimo de k vertices
// leer archivo
// grafica en matriz de adyacencias
// floyd-warshall
// completar grafica
// arbol de peso minimo
// funcion de costo

impl GIO {
    pub fn new(grafica: Grafica) -> Self {
        
    }
    pub fn generar_promedios(&self mut) {
        
        for (vertice_nombre, vertice) in self.grafica.vertices.iter() {
            let mut peso_total : f64 = 0.0;
            let mut numero_vecinos :f64 = 0.0; 
            for (peso_vecino, _) in vertice.vecinos.iter() { 
                peso_total += peso_vecino;
                numero_vecinos += 1.0;
            }
            self.promedios.insert(vertice_nombre, (peso_total/numero_vecinos));
        }
    }

    pub fn floyd_warshall(&self mut) {
        let mut i:usize = 0;
        let mut correspondencia = HashMap::<i64, String>::new();
        for (vertice_nombre, _) in self.grafica.vertices.iter() {
            correspondencia.insert(i,vertice_nombre);
        }
        let mut arr_warshall:Vec<i64> = Vec![-1; self.grafica.num_vertices*self.grafica.num_vertices];

        i = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;
        while k < self.grafica.num_vertices {
            while i < self.grafica.num_vertices {
                while j < self.grafica.num_vertices{
                    if i == j || j == k || k == i {
                        continue;
                    }
                    if Some(original) = self.grafica.get_peso(correspondencia.get(i).unwrap(), correspondencia.get(j).unwrap()) {
                        continue;
                    } else {
                        let tmp;
                        if Some(caminoA) = self.grafica.get_peso(correspondencia.get(i).unwrap(), correspondencia.get(j).unwrap()) && Some(caminoB) = self.grafica.get_peso(correspondencia.get(k).unwrap()) {
                            tmp = caminoA + caminoB;
                        } else if Some(caminoA) = self.grafica.get_peso(correspondencia.get(i).unwrap()) {
                            
                        }
                    }
                }
            }
        }
    }
}
