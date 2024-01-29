use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Paquete {
    pub nombre: String,
    pub version: String,
    pub arquitectura: String,
}

#[derive(Clone,Debug, Deserialize)]
pub struct PaqueteComp {
    #[serde(default)]
    pub nombre: String,
    pub version: String,
    pub arquitectura: String,
    pub descripcion: String,
    pub fecha: String,
}

impl Paquete {
    pub fn str_a_paquete(cadena: &str, arquitectura:&str) -> Paquete {
        let partes: Vec<&str> = cadena.split('=').collect();
    
        let (mut nombre, version) = match partes.as_slice() {
            [nombre, version] => (nombre.to_string(), version.to_string()),
            [nombre] => (nombre.to_string(), "release".to_string()),
            _=> panic!("Formato no valido")
        };
        if nombre == "*" { nombre = "release".to_string(); }
        Paquete { nombre, version, arquitectura:arquitectura.to_string(), } 
    }
}