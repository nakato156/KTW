use std::path::PathBuf;
use serde_json::Value;

use crate::paquete::{Paquete, PaqueteComp};

pub fn obtener_path_home() -> Option<String> {
    dirs::home_dir().map(|path| path.to_string_lossy().into_owned())
}

pub fn obtener_path_package() -> String {
    let path_home = obtener_path_home().unwrap_or(".".into());
    format!("{path_home}/ktw/Packages.json")
}

pub fn obtener_paquetes(all_versions: Option<bool>) -> Vec<PaqueteComp> {
    let path_package = obtener_path_package();
    let json = std::fs::read_to_string(path_package).unwrap_or("{}".into());
    let json_data: Value = serde_json::from_str(&json).expect("Error al leer los paquetes");
    
    let mut todos_los_paquetes = Vec::new();
    
    // Itera sobre todas las claves dentro de "Paquetes"
    if let Some(paquetes) = json_data.get("Paquetes").and_then(Value::as_object) {
        for (nombre_paquete, paquete_value) in paquetes {
            // Deserializa el vector de paquetes correspondiente a la clave actual
            if let Ok(mut paquetes_paquete) = serde_json::from_value::<Vec<PaqueteComp>>(paquete_value.clone()) {
                // Agrega los paquetes al vector general
                if let Some(_) = all_versions {
                    for paquete in &mut paquetes_paquete {
                        paquete.nombre = nombre_paquete.to_string();
                    }
                    todos_los_paquetes.extend(paquetes_paquete);
                }
                else{
                    todos_los_paquetes.push(paquetes_paquete.pop().unwrap());
                }
            } else {
                eprintln!("Error al deserializar paquetes para '{}'", nombre_paquete);
            }
        }
    } else {
        eprintln!("Clave 'Paquetes' no encontrada en el JSON");
    }

    todos_los_paquetes
}

pub fn formato_nombre_archivo(paquete: &Paquete) -> String {
    format!(
        "{nombre}-{version}-{win}.exe",
        nombre = paquete.nombre,
        version = paquete.version,
        win = "64"
    )
}

pub fn obtener_path_paquete(paquete: &Paquete) -> PathBuf  {
    let path = obtener_path_home().unwrap_or(".".into());
    let filename = formato_nombre_archivo(paquete);
    let path_folder = format!("{path}/ktw/{nombre}", nombre = paquete.nombre);
    PathBuf::from(format!("{path_folder}/{filename}").as_str())
}

pub fn obtener_dir_paquete(paquete: &Paquete) -> String {
    let path = obtener_path_home().unwrap_or(".".into());
    format!("{path}/ktw/{nombre_paquete}", nombre_paquete = paquete.nombre)
}
