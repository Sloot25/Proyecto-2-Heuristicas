use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;

use ordered_float::OrderedFloat;

use pheap::PairingHeap;

struct Arista {
    padre: usize,
    hijo : usize,
}

pub struct Grafica {
    pub vertices: Vec<f64>,
    pub peso_arbol_generador: f64,
    pub size:usize,
    pesos: Vec<f64>,
}

impl Grafica {

    pub fn new(size: usize) -> Self {
        Grafica{
            vertices: vec![0.0; size*size],
            peso_arbol_generador: 0.0,
            size: size,
            pesos: Vec::<f64>::new(),
        }
    }
    
    pub fn to_string(&mut self) -> String {
        let mut res:String = "[ ".to_string();
        for x in 0..self.size {
            for y in 0..self.size{
                res += &((self.vertices[self.size * x + y]).to_string() + ", ");
            }

            res += "]\n[ ";
        }
        return res;
    }

    pub fn agregar_arista(&mut self, a:usize, b: usize, p:f64) {
        self.vertices[a*self.size + b] = p;
        self.vertices[b*self.size + a] = p;
        self.pesos.push(p);
    }
    
/// Recibe los vertices que forman parte del arbol para aristas 
    pub fn arbol_generador_minimo(&mut self, vertices_del_arbol:HashSet<usize>, vertice_inicial: usize, k: usize) ->Grafica {
        let mut arbol = Grafica::new(self.size);
        let mut heap = PairingHeap::new();
        let mut visitados:Vec<bool> = vec![false; self.size];
        let mut n:usize = 0;
        let mut peso_almacenado:Vec<f64> = vec![f64::INFINITY; self.size];

        visitados[vertice_inicial] = true;
        
        for vecino in 0..self.size {
            if vertices_del_arbol.contains(&vecino) && visitados[vecino] == true{
                continue;
            }
            let peso = self.vertices[vertice_inicial*self.size + vecino];
            if peso > 0.0 {
                
                heap.insert(Arista{
                    padre: vertice_inicial,
                    hijo: vecino,
                }, OrderedFloat(peso))
            }
        }
        
        while let Some((vertice, peso)) = heap.delete_min(){
            if visitados[vertice.hijo] {
                continue;
            }

            visitados[vertice.hijo] = true;
            arbol.agregar_arista(vertice.padre, vertice.hijo, peso.into_inner() );
            n += 1;

            if n >= k-1 {
                return arbol;
            }
            
            for vecino in 0..self.size {
                if vertices_del_arbol.contains(&vecino) && visitados[vecino] == false {
                    let peso_vecino = self.vertices[vertice.hijo*self.size + vecino];
                    if peso_vecino > 0.0{
                        heap.insert(Arista{
                            hijo: vecino,
                            padre: vertice.hijo
                        }, OrderedFloat(peso_vecino))
                    }
                }
            }
        } 

        return arbol;
            
       
    }
        
}
