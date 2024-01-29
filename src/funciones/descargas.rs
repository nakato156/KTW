use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::process::Command;
use crate::funciones::helpers::*;

use crate::paquete::Paquete;

const HOST:&str = "https://ktw.onrender.com/";


pub async fn descargar_recurso(paquete: Paquete) -> Result<(), Box<dyn Error>> {
    let mut map = HashMap::new();
    let (nombre, version, win) = (&paquete.nombre, &paquete.version, &paquete.arquitectura);
    map.insert("version".to_string(), version.clone());
    map.insert("win".to_string(), win.clone());

    let url = format!("{HOST}/repo/install/{}", nombre);
    let client = Client::new();
    let res = client.post(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&map)?)
        .send()
        .await?;
    
    if res.status().is_success() {
        let content_length = res.content_length().unwrap_or(0);

        let pb = ProgressBar::new(100);
        pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"));
        
        let reader = res.bytes().await?;
        let mut downloaded_bytes = 0;
        
        let path_paquete = obtener_path_paquete(&paquete);
        if !path_paquete.parent().unwrap().exists() {
            create_dir_all(path_paquete.parent().unwrap())?;
        }

        let mut file = File::create(path_paquete)?; 

        for chunk in reader {
            downloaded_bytes += 1;
            pb.set_position((downloaded_bytes * 100) / content_length);
            file.write_all(&[chunk])?;
        }
        
        pb.finish_with_message("done");
        Ok(())
    }
    else {
        Err(format!("Error:{}", res.status()).into())
    }
}

pub fn desinsalar_recurso(paquete: &Paquete) -> Option<std::io::Error>{
    let path_paquete = obtener_dir_paquete(paquete);
    let path_uninstall = format!("{path_paquete}/uninstall.ps1");
    
    Command::new(&path_uninstall).output().err()
}