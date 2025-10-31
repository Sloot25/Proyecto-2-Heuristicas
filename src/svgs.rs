use crate::Grafica;
use std::collections::HashMap;
use std::io::Write;
use std::fs::File;


pub fn construir_svgs (grafica : Grafica, ruta : String, toID: HashMap<usize, String> ) {
    let mut svg_text: String = "".to_string();
    let mut svg_aritas: String = "".to_string();

    
    let vertice = grafica.mayor_grado();
    let ancho = (grafica.ancho() * 500) as f64;
    let ancho_arbol = (grafica.ancho() * 40) as f64;
    let alto = (grafica.altura() * 500) as f64; 
    let mut posiciones = HashMap::<usize, (f64, f64)>::new();
    let mut nivel = 1;
    let mut heap = Vec::new();
    let mut heap_2 = Vec::new();
    let mut set = vec![0; grafica.size]; 
    set[vertice] = 1;
    
    for x in 0..grafica.size {
        let hijo = grafica.vertices[vertice * grafica.size + x];
        if hijo != 0.0 && set[x] == 0 {
            heap.push((vertice,x));
            set[x] = 1; 
        }
    }

    svg_text =format!("{}<circle cx='{}' cy='{}' r='25' fill='white' stroke='black' stroke-width='2.5' />\n", svg_text, (ancho/2.0), 30);
    svg_text =format!("{}<text fill='black' font-family='sans-serif' font-size='18' x='{}' y='{}' text-anchor='middle' > {} </text>", svg_text, (ancho/2.0), 35, vertice);

    posiciones.insert(vertice, (ancho/2.0, 30.0));
    let mut cuentita = 0.0;  
    while heap.len() != 0 {
        let (padre, hijo) = heap.remove(0);
        let grado_hijo = grafica.grado_vertice(hijo) as f64;
        let grado_padre = grafica.grado_vertice(padre) as f64; 
        let porcentaje = grado_hijo / grado_padre;
        cuentita += porcentaje;
        let pos_x = cuentita * ancho_arbol;
        let (padre_x, padre_y) = posiciones.get(&padre).unwrap();
        let pos_y = padre_y + 50.0;
        
        svg_text = format!("{}<circle cx='{}' cy='{}' r='25' fill='white' stroke='black' stroke-width='2.5' />\n", svg_text, pos_x, pos_y);
        svg_text =format!("{}<text fill='black' font-family='sans-serif' font-size='18' x='{}' y='{}' text-anchor='middle' > {} </text>", svg_text, pos_x, pos_y + 5.0 , hijo);
        svg_aritas = format!("{}<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='black' stroke-width='3' />\n", svg_aritas, padre_x, padre_y, pos_x, pos_y);

        posiciones.insert(hijo, (pos_x, pos_y));
        
        for x in 0..grafica.size {
            let th = grafica.vertices[hijo * grafica.size + x];
            if th != 0.0 && set[x] == 0 {
                heap_2.push((hijo,x));
                set[x] = 1; 
            }
        }
        if heap.len() == 0 {
            nivel = nivel + 1;
            heap = heap_2.clone();
            heap_2 = Vec::new();
            cuentita = 0.0;
        }
    }

    let svg = format!("<?xml version='1.0' encoding='UTF-8' ?>\n <svg width='{}' height='{}' >\n <g>\n{}{}</g>\n</svg>\n", ancho, alto , svg_aritas, svg_text);

    let mut file = File::create(format!("{}.svg", ruta));
    let _ = file.expect("REASON").write_all(svg.as_bytes());

}



