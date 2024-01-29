mod funciones;
mod comandos;
mod paquete;

use comandos::{instalar, actualizar, desinstalar, convertir_a_paquetes, exportar};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="KTW", author="Nakato", version="0.1", about, long_about = None)]
struct Args {
    comando: String,
    paquetes: Vec<String>,

    #[clap(long, help = "Versión de Windows: x64 o x32", default_value="x64")]
    win: String,

    #[clap(long, help = "Para exportar todas las veriones de un programa", default_value=None)]
    all: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let comando = args.comando.as_str();
    
    if args.paquetes.len() == 0 && args.comando != "export" {
        println!("Necesita proprocionar un nombre de un paquete");
        return;
    }

    let paquetes = convertir_a_paquetes(args.paquetes.clone(), args.win.as_str());
    match comando {
        "install" => {
            instalar(&paquetes).await;
            funciones::obtener_paquetes(None);
        },
        "update" => {
            actualizar(&paquetes).await;
        },
        "uninstall" => {
            desinstalar(&paquetes).await;
        },
        "export" => {
            let res = exportar(Some(args.all));
            match res {
                Ok(_) => { },
                Err(err) => {
                    println!("No se pudo exportar los programas\n{}", err);
                }
            }
        },
        _ => {
            println!("Comando '{comando}' no reconocido");
        }
    }

}
