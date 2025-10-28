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
#[derive(Clone)]
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
        self.peso_arbol_generador += p;
    }
    

    pub fn generarAdyacencias(&mut self, toID: HashMap<usize, String>){
        let mut v = Vec::<(String,String, f64)>::new();
        for x in 0..self.size {
            for y in (x+1)..self.size {
                let peso = self.vertices[self.size * x + y];
                if peso != 0.0 {
                    v.push((toID.get(&x).unwrap().to_string(),toID.get(&y).unwrap().to_string(),peso));
                }
            }
        }
        let vector : Vec<_> = v.iter().collect();

        println!("{:?}",vector);
    }


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

            if !vertices_del_arbol.contains(&vertice.hijo) {
                continue;
            }

            visitados[vertice.hijo] = true;
            arbol.agregar_arista(vertice.padre, vertice.hijo, peso.into_inner() );
            
            
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::constructor_grafica::Constructor_grafica;
    use crate::HashSet;
    use crate::PeSOA;

    fn construir_grafica() -> Grafica {
        let mut constructor =  Constructor_grafica::new("archivo.txt".to_string());
        let mut g = constructor.cargar_datos();
        return g;
    }
    
    #[test]
    fn ok_arbol_generador_minimo_sin_completar(){
        let mut arbol : Grafica = Grafica::new(5);
        let mut constructor =  Constructor_grafica::new("archivo.txt".to_string());
        let mut g = constructor.cargar_datos();
        arbol.agregar_arista(*constructor.id_to_int.get("B").unwrap(),*constructor.id_to_int.get("C").unwrap(),2.0);
        arbol.agregar_arista(*constructor.id_to_int.get("B").unwrap(),*constructor.id_to_int.get("D").unwrap(), 3.0);
        let mut h = HashSet::<usize>::new();
        h.insert(*constructor.id_to_int.get("B").unwrap());
        h.insert(*constructor.id_to_int.get("C").unwrap());
        h.insert(*constructor.id_to_int.get("D").unwrap());
        assert_eq!(g.arbol_generador_minimo(h,*constructor.id_to_int.get("D").unwrap(),3).vertices, arbol.vertices);
    }

    #[test]
    fn ok_arbol_generador_minimo_completado(){
        let mut arbol : Grafica = Grafica::new(5);
        let mut constructor =  Constructor_grafica::new("archivo.txt".to_string());
        let mut g = constructor.cargar_datos();
        let mut vector = Vec::<f64>::new();
        vector.push(2.0);
        vector.push(3.0);
        vector.push(4.0);
        vector.push(5.0);
        vector.push(7.0);
        vector.push(9.0);
        let mut heuristica = PeSOA::new(g,vector, 3,0);
        heuristica.completar_grafica();
        
        arbol.agregar_arista(*constructor.id_to_int.get("B").unwrap(),*constructor.id_to_int.get("C").unwrap(),2.0);
        arbol.agregar_arista(*constructor.id_to_int.get("B").unwrap(),*constructor.id_to_int.get("D").unwrap(), 3.0);
        let mut h = HashSet::<usize>::new();
        h.insert(*constructor.id_to_int.get("B").unwrap());
        h.insert(*constructor.id_to_int.get("C").unwrap());
        h.insert(*constructor.id_to_int.get("D").unwrap());
        assert_eq!(heuristica.grafica.arbol_generador_minimo(h,*constructor.id_to_int.get("D").unwrap(),3).vertices, arbol.vertices);

    }
}
