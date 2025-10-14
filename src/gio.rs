use crate::HashSet;
use crate::Grafica;

pub struct GIO {
    pub solucion_actual: HashSet<usize>,
    pub peso_solucion_actual: f64,
    floyd: Vec<f64>,
    grafica: Grafica,
    diametro_grafica: f64,
    normalizador: f64,
    k: usize
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
    pub fn new( grafica: Grafica, pesos_aristas: Vec<f64>, k : usize) -> Self {
        let normalizador = Self::calcular_normalizador(pesos_aristas, k);
        GIO {
            solucion_actual: HashSet::<usize>::new(),
            peso_solucion_actual: 0.0,
            grafica: grafica,
            diametro_grafica: 0.0,
            normalizador: normalizador,
            floyd: Vec::<f64>::new(),
            k: k,
        }
    }

    fn buscar_diametro(&mut self) {
        let mut maximo = 0.0;
        for x in 0..self.grafica.size {
            for y in 0..self.grafica.size {
                if maximo < self.floyd[x*self.grafica.size + y] {
                    maximo = self.floyd[x*self.grafica.size + y];
                }
            }
        }
        self.diametro_grafica = maximo;
    }

    fn calcular_normalizador(pesos_aristas: Vec<f64>, k: usize) -> f64 {
        let mut i:usize = pesos_aristas.len() - k + 1;
        let mut normalizador : f64 = 0.0;
        while i < pesos_aristas.len() {
            normalizador = normalizador + pesos_aristas[i];
            i = i + 1;
        }
        return normalizador;
        
    }
    
    fn floyd_warshall(&mut self) {
        let mut caminos = self.grafica.vertices.clone();

        for k in 0..self.grafica.size {
            for i in 0..self.grafica.size {
                for j in 0..self.grafica.size {
                    let dt = caminos[i*self.grafica.size + k] + caminos[self.grafica.size*k + j];
                    if i == j {
                        continue;
                    }
                    if caminos[i * self.grafica.size + j] > dt || caminos[i*self.grafica.size + j] == 0.0 {
                        caminos[i * self.grafica.size + j] = dt;
                    } 
                }
            }
        }
        self.floyd = caminos;
    }

    pub fn completar_grafica(&mut self) {
        self.floyd_warshall();
        self.buscar_diametro();
        for i in 0..self.grafica.size {
            for j in 0..self.grafica.size {
                if self.grafica.vertices[i * self.grafica.size + j] == 0.0{
                    self.grafica.vertices[i * self.grafica.size + j] = self.diametro_grafica * self.floyd[i * self.grafica.size + j];
                }
            }
        }
        println!("{}", self.grafica.to_string())
    }

    fn calcular_peso(&mut self) -> f64 {
        return (self.grafica.peso_arbol_generador/self.normalizador)
    }
}
