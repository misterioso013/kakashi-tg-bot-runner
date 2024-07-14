use std::env::consts::OS;
use std::io::Error;
use std::process::Command;
use std::process::Output;

pub fn unzip_file(file: &str) {
    let unzip_command: &str = match OS {
        "windows" => "tar -xf",
        "linux" => "unzip",
        "macos" => "unzip",
        _ => "unzip",
    };
    let unzip: Result<Output, Error> = Command::new(unzip_command).arg(file).output();
    match unzip {
        Ok(output) => {
            // Se o comando foi executado com sucesso, faça algo com o output aqui
            print!("Result: {}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            // Se houve um erro ao executar o comando, faça algo com o erro aqui
            println!(
                "Erro ao executar o comando: ({}) erro: {}",
                unzip_command, e
            );
        }
    }
}
