use std::fs::read_to_string;
use std::collections::HashMap;
use std::fmt::Write;
use std::collections::BTreeSet;

use crate::Grafica;

pub struct Constructor_grafica {
    pub id_to_int: HashMap<String, usize>,
    pub int_to_id: HashMap<usize, String>,
    archivo: String,
    pub pesos_aristas: Vec<f64>,
}

impl Constructor_grafica {

    pub fn new(archivo:String) -> Self{
        Constructor_grafica{
            id_to_int: HashMap::new(),
            int_to_id: HashMap::new(),
            archivo: archivo,
            pesos_aristas: Vec::<f64>::new(),
        }
    }
    
    pub fn cargar_datos(&mut self) -> Grafica{
        let contenido:String = read_to_string(self.archivo.clone()).unwrap();
        let mut visitado = BTreeSet::<String>::new();
        let lineas: Vec<&str> = contenido.lines().collect();
        let mut size:usize = 0;
    
        for linea in lineas.iter() {
            let datos: Vec<&str> = linea.split(",").collect();
            if !visitado.contains(datos[0]) {
                visitado.insert(datos[0].to_string());
                self.id_to_int.insert(datos[0].to_string(), size);
                self.int_to_id.insert(size,datos[0].to_string());
                size += 1;
            }

            if !visitado.contains(datos[1]) {
                visitado.insert(datos[1].to_string());
                self.id_to_int.insert(datos[1].to_string(), size);
                self.int_to_id.insert(size, datos[1].to_string());
                size += 1;
            }
        }
    
        let mut g:Grafica = Grafica::new(size);
        
        for linea in lineas.iter() {
            let datos: Vec<&str> = linea.split(",").collect();
            let origen = self.id_to_int.get(datos[0]).unwrap();
            let destino = self.id_to_int.get(datos[1]).unwrap();
            let peso = datos[2].parse::<f64>().unwrap();
            g.agregar_arista(*origen, *destino, peso);
            self.pesos_aristas.push(peso);
        }

        self.pesos_aristas.sort_by(|a,b| a.partial_cmp(b).unwrap());
        g
    }

}
