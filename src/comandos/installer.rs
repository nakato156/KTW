use crate::funciones::{descargar_recurso, desinsalar_recurso, obtener_paquetes};
use crate::paquete::Paquete;
use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

// funciones principales
pub async fn instalar_actualizar(paquetes: &Vec<Paquete>, mensaje: &str) -> bool {
    for paquete in paquetes.iter() {
        println!(
            "{}",
            format!(
                "{mensaje} {} {nombre}",
                "el paquete",
                nombre = paquete.nombre
            )
            .yellow()
        );
        let res = descargar_recurso((*paquete).clone()).await;
        match res {
            Ok(()) => (),
            Err(err) => {
                println!(
                    "{}\n{err_}",
                    "No se pudo instalar el paquete".red(),
                    err_ = err.to_string().red()
                );
                return false;
            }
        }
    }
    true
}
pub async fn instalar(paquetes: &Vec<Paquete>) -> bool {
    instalar_actualizar(paquetes, "Instalando").await
}

pub async fn actualizar(paquetes: &Vec<Paquete>) -> bool {
    instalar_actualizar(paquetes, "Actualizando").await
}

pub async fn desinstalar(paquetes: &Vec<Paquete>) -> bool {
    for paquete in paquetes.iter() {
        if let Some(res) = desinsalar_recurso(paquete) {
            println!(
                "{}{}",
                "No se pudo ejecutar el desinstalador\n".red(),
                res.to_string().red()
            );
        }
    }
    true
}

pub fn exportar(all_version: Option<bool>) -> io::Result<()> {
    let paquetes_str: Vec<String> = obtener_paquetes(all_version)
        .iter()
        .map(|paquete| {
            String::from(format!(
                "\"{}-{}=={}\"",
                paquete.nombre, paquete.arquitectura, paquete.version
            ))
        })
        .collect();

    let lista_pwsh_paquetes = format!("$paquetes = {}", paquetes_str.join(", "));

    let original_file_path = "temp.ps1";
    let new_file_path = "./instalador.ps1";

    // Abre el archivo original en modo de lectura
    let original_file = File::open(&original_file_path)?;
    let reader = BufReader::new(&original_file);

    // Abre un nuevo archivo en modo de escritura para la copia
    let new_file = File::create(&new_file_path)?;
    let mut writer = BufWriter::new(new_file);
    
    writer.write_all(lista_pwsh_paquetes.as_bytes())?;
    reader.lines().skip(1).for_each(|f| {
        writer.write_all(f.unwrap().as_bytes()).unwrap();
        writer.write_all("\n".as_bytes()).unwrap();
    });
    Ok(())
}

// funciones de procesamiento
pub fn convertir_a_paquetes(paquetes: Vec<String>, win: &str) -> Vec<Paquete> {
    paquetes
        .iter()
        .map(|paquete| Paquete::str_a_paquete(paquete, win))
        .collect()
}
