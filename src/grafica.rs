use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::BTreeSet;
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

    pub fn mayor_grado(&self) -> usize {
        let mut vertice_maximo_grado = 0;
        let mut maximo_grado = 0;
        for x in 0..self.size {
            let mut contador = 0;
            for y in 0..self.size {
                if self.vertices[x * self.size + y] != 0.0 {
                    contador += 1;
                }
            }
            if contador > maximo_grado {
                maximo_grado = contador;
                vertice_maximo_grado = x;
            }
        }
        return vertice_maximo_grado;
    }

    pub fn ancho(&self) -> i64 {
        let mut maximo_grado = 0;
        for x in 0..self.size {
            let mut contador = 0;
            for y in 0..self.size  {
                if self.vertices [ x * self.size + y] != 0.0 {
                    contador += 1; 
                }
            }
            if contador > maximo_grado {
                maximo_grado = contador;
            }
        }
        return maximo_grado;
    }
    
    pub fn altura(&self) -> i64 {
        let mut vertice = self.mayor_grado();
        let mut set = vec![0; self.size];
        let mut heap = Vec::new();
        let mut heap_2 = Vec::new();
        let mut count = 1;
        set[vertice] = 1;
        while heap.len() != 0 {
            let removed = heap.remove(0);
            for x in 0..self.size {
                let hijo = self.vertices[removed * self.size + x];
                if hijo != 0.0 && set[x] == 0 {
                    set[x] = 1;
                    heap_2.push(x);
                }
            }
            if heap.len() == 0 {
                heap = heap_2.clone();
                heap_2 = Vec::new();
                count += 1;
            }
        }

        return count;
    }

    pub fn bfs_ses(&self, toID: HashMap<usize, String>) {
        let mut vertice = self.mayor_grado();
        let mut set = vec![0; self.size];
        let mut heap = Vec::new();
        let mut heap_2 = Vec::new();
        heap.push(vertice);
        set[vertice] = 1;
        while heap.len() != 0 {
            let removed = heap.remove(0);
            print!("{:?},", toID.get(&removed).unwrap());
            for x in 0..self.size {
                let hijo = self.vertices[removed * self.size + x];
                if hijo != 0.0 && set[x] == 0 {
                    heap_2.push(x);
                    set[x] = 1;
                }
            }
            if heap.len() == 0 {
                println!();
                heap = heap_2.clone();
                heap_2 = Vec::new();
            }
        }
        
    }

    pub fn grado_vertice(&self, vertice: usize) -> i64 {
        let mut count = 0;
        for y in 0..self.size {
            let vec = self.vertices[vertice * self.size + y];
            if vec != 0.0 {
                count = count + 1;
            }
        }
        return count;
    }
    
    pub fn arbol_generador_minimo(&mut self, vertices_del_arbol:BTreeSet<usize>, vertice_inicial: usize, k: usize) ->Grafica {
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
    use crate::BTreeSet;
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
        let mut h = BTreeSet::<usize>::new();
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
        let mut h = BTreeSet::<usize>::new();
        h.insert(*constructor.id_to_int.get("B").unwrap());
        h.insert(*constructor.id_to_int.get("C").unwrap());
        h.insert(*constructor.id_to_int.get("D").unwrap());
        assert_eq!(heuristica.grafica.arbol_generador_minimo(h,*constructor.id_to_int.get("D").unwrap(),3).vertices, arbol.vertices);

    }
}
