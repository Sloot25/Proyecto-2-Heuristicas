use crate::Grafica;
use std::collections::BTreeSet;
use rand::prelude::IndexedRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng;


#[derive(Clone)]
pub struct Pinguino {
    pub solucion: BTreeSet<usize>,
    pub fitness: f64,
    pub identificador: usize,
}

pub struct GrupoPinguinos {
    pub pinguinos: Vec<Pinguino>,
    pub mejor_pinguino: Option<Pinguino>,
}

pub struct PeSOA {
    grupos : Vec<GrupoPinguinos>,
    pub mejor_pinguino_actual: Option<Pinguino>,
    mejor_solucion: f64,
    floyd: Vec<f64>,
    pub grafica: Grafica,
    pub diametro_grafica: f64,
    pub normalizador: f64,
    k: usize,
    random: StdRng,
}

impl PeSOA {
    pub fn new( grafica: Grafica, pesos_aristas: Vec<f64>, k : usize, semilla: i64) -> Self {
        let normalizador = Self::calcular_normalizador(pesos_aristas, k);
        let rng = StdRng::seed_from_u64(semilla as u64);
        PeSOA {
            grupos: Vec::new(),
            mejor_pinguino_actual: None,
            mejor_solucion: 0.0,
            grafica: grafica,
            diametro_grafica: 0.0,
            normalizador: normalizador,
            floyd: Vec::<f64>::new(),
            k: k,
            random:rng,
        }
    }

    pub fn iniciar_PeSOA(&mut self, num_pinguinos: usize, num_grupos: usize) {
        let mut pinguinos = Vec::new();
        let todos : Vec<usize> = (0..self.grafica.size).collect();
        for i in 0..num_pinguinos {
            let sol : BTreeSet<usize> = todos.choose_multiple(&mut self.random, self.k).cloned().collect();
            let arbol = self.grafica.arbol_generador_minimo(sol.clone(), *sol.iter().next().unwrap(), self.k);
            let fit = self.calcular_peso(arbol);
            pinguinos.push(Pinguino {
                solucion: sol,
                fitness: fit,
                identificador:i,
            });
        }

        let pinguinos_por_grupo = (num_pinguinos as f64/ num_grupos as f64).ceil() as usize;
        for chunk in pinguinos.chunks(pinguinos_por_grupo) {
            self.grupos.push(GrupoPinguinos {
                pinguinos: chunk.to_vec(),
                mejor_pinguino: None,
            });
        }

        self.actualizar_pesos();
    }

    fn iterar(&mut self, clavados: usize) {
        let normalizador = self.normalizador;
        
        for grupo in &mut self.grupos {
            for pinguino in &mut grupo.pinguinos {
                for _ in 0..clavados {
                    let mut nueva = pinguino.solucion.clone();
                    if let Some(&vertice_quitar) = pinguino.solucion.iter().collect::<Vec<_>>().choose(&mut self.random) {
                        nueva.remove(&vertice_quitar);
                        let mut vertice_aniadir = None;

                        if self.random.gen_range(0.0..1.0) < 0.2 {
                            if let Some(mejor_local) = &grupo.mejor_pinguino {
                                let candidatos:Vec<usize> = mejor_local.solucion.iter().filter(|&&v| !pinguino.solucion.contains(&v))
                                    .cloned().collect();
                                if !candidatos.is_empty() {
                                    vertice_aniadir = candidatos.choose(&mut self.random).cloned();
                                }
                            }
                        }
                        
                        if vertice_aniadir.is_none() {
                            let candidatos : Vec<usize> = (0..self.grafica.size).filter(|v| !pinguino.solucion.contains(v)).collect();
                            
                            vertice_aniadir = candidatos.choose(&mut self.random).copied();
                        }

                        if let Some(v_aniadir) = vertice_aniadir {
                            nueva.insert(v_aniadir);
                        } else {
                            nueva.insert(*vertice_quitar);
                        }
                        
                    }
                    let arbol = self.grafica.arbol_generador_minimo(nueva.clone(), *nueva.iter().next().unwrap(), self.k);
                    let fit = arbol.peso_arbol_generador / normalizador;
                    if fit < pinguino.fitness {
                        pinguino.solucion = nueva;
                        pinguino.fitness = fit;
                    }
                }
            }
        }

        self.actualizar_pesos();

        self.grupos.sort_by(|a,b|
                            a.mejor_pinguino.as_ref().map_or(f64::INFINITY, |p| p.fitness)
                            .partial_cmp(&b.mejor_pinguino.as_ref().map_or(f64::INFINITY, |p| p.fitness))
                            .unwrap()
        );

        if self.grupos.len() > 1 {
            if let Some(mejor_pinguino_global) = self.mejor_pinguino_actual.clone() {
                if let Some(peor_pinguino) = self.grupos[0].pinguinos.iter_mut().max_by(|a,b| a.fitness.partial_cmp(&b.fitness).unwrap()) {
                    peor_pinguino.solucion = mejor_pinguino_global.solucion;
                    peor_pinguino.fitness = mejor_pinguino_global.fitness;
                }
            }
        }

        self.actualizar_pesos();
    }

    pub fn run_pesoa(&mut self,niveles : usize, clavados: usize, epsilon: f64) {
        let mut actual : f64 = 0.0;
        let mut i = 0;
        while niveles > i {
            self.iterar(clavados);
            if let Some(mejor) = &self.mejor_pinguino_actual {
                if (actual - mejor.fitness).abs() < epsilon{
                    i = i + 1;
                }
                actual = mejor.fitness;
                println!("Iteracion {}: Mejor peso = {}", i , mejor.fitness);
            }
        }

        self.barrido();
        println!("Solucion postBarrido: {}", self.mejor_pinguino_actual.clone().unwrap().fitness);
    }
    
    fn actualizar_pesos(&mut self) {
        let mut min_global = f64::INFINITY;
        let mut candidato_global = None;
        for grupo in &mut self.grupos {
            let mut min = f64::INFINITY;
            let mut candidato = None;
            for p in &grupo.pinguinos {
                if p.fitness < min {
                    candidato = Some(p.clone());
                    min = p.fitness;
                }
                //println!("Identificador {}, Costo {}",p.identificador, p.fitness);
            }
            grupo.mejor_pinguino = candidato;
            if let Some(mejor_local) = &grupo.mejor_pinguino {
                if mejor_local.fitness < min_global {
                    min_global = mejor_local.fitness;
                    candidato_global = Some(mejor_local.clone());
                }
            }
        }
        
        self.mejor_pinguino_actual = candidato_global;
        println!("Mejor peso = {}", self.mejor_pinguino_actual.clone().unwrap().fitness);
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
            println!("A: {}", pesos_aristas[i]);
            normalizador = normalizador + pesos_aristas[i];
            i = i + 1;
        }
        return normalizador;
        
    }

    pub fn barrido(&mut self) {
        let mut mejoro = true;

        while mejoro {
            mejoro = false;
            if let Some(mejor) = &mut self.mejor_pinguino_actual {
                let candidatos : Vec<usize> = (0..self.grafica.size)
                    .filter(|v| !mejor.solucion.contains(v))
                    .collect();
                let solucion_a_iterar : Vec<usize> = mejor.solucion.iter().cloned().collect();
                
                'outer: for &v_remove in &solucion_a_iterar {
                    let mut nueva_base = mejor.solucion.clone();
                    nueva_base.remove(&v_remove);

                    for &v_add in &candidatos {
                        let mut nueva_sol = nueva_base.clone();
                        nueva_sol.insert(v_add);

                        let arbol = self.grafica.arbol_generador_minimo (
                            nueva_sol.clone(),
                            *nueva_sol.iter().next().unwrap(),
                            self.k
                        );
                        let fit = arbol.peso_arbol_generador / self.normalizador;

                        if fit < mejor.fitness {
                            mejor.solucion = nueva_sol;
                            mejor.fitness = fit;
                            mejoro = true;
                            println!("fit nuevo: {}", fit);
                            break 'outer; 
                        }
                    }
                }
            } else {
                break;
            }
        }            
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
        let mut pesada = 0.0;
        for i in 0..self.grafica.size {
            for j in 0..self.grafica.size {
                if self.grafica.vertices[i * self.grafica.size + j] == 0.0{
                    self.grafica.vertices[i * self.grafica.size + j] = self.diametro_grafica * self.floyd[i * self.grafica.size + j] * (self.k as f64);
                }
                if pesada < self.grafica.vertices[i * self.grafica.size + j] {
                    pesada = self.grafica.vertices[i*self.grafica.size + j];
                }
            }
        }
        println!("P: {}", pesada);
    }

    fn calcular_peso(&self, g: Grafica) -> f64 {
        return g.peso_arbol_generador/self.normalizador
    }

    
}
/*
#[cfg(test)]

mod tests {
    use super::*;
    use crate::constructor_grafica::Constructor_grafica;
    use crate::BTreeSet;

    #[test]
    fn ok_pesoa_conexa(){
        let mut constructor = Constructor_grafica::new("archivo.txt".to_string());
        let mut g = constructor.cargar_datos();
        let mut vector = Vec::<f64>::new();
        vector.push()
    }
}
*/
