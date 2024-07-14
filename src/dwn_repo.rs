use crate::unzip::unzip_file;
use std::env::consts::OS;
use std::fs::File;
use std::io::copy;
use std::io::Error;
use std::process::Command;
use std::process::Output;

pub fn download_repository(url: &str, repo_name: &str, branch: &str) {
    let mut response = reqwest::blocking::get(url).unwrap();
    let mut out = File::create(format!("{}.zip", repo_name)).unwrap();
    copy(&mut response, &mut out).unwrap();
    println!("Download finalizado!");
    unzip_file(format!("{}.zip", repo_name).as_str());
    // remover o arquivo zip
    let remove_command: &str = match OS {
        "windows" => "del",
        "linux" => "rm",
        "macos" => "rm",
        _ => "rm",
    };
    let remove_file: Result<Output, Error> = Command::new(remove_command)
        .arg(format!("{}.zip", repo_name))
        .output();
    match remove_file {
        Ok(output) => {
            // Se o comando foi executado com sucesso, faça algo com o output aqui
            print!("Result: {}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            // Se houve um erro ao executar o comando, faça algo com o erro aqui
            println!(
                "Erro ao executar o comando: ({}) erro: {}",
                remove_command, e
            );
        }
    }

    // mover os arquivos para a pasta .bot
    let move_command: &str = match OS {
        "windows" => "move",
        "linux" => "mv",
        "macos" => "mv",
        _ => "mv",
    };
    let move_file: Result<Output, Error> = Command::new(move_command)
        .arg(format!("{}-{}", repo_name, branch))
        .arg(".bot")
        .output();
    match move_file {
        Ok(_) => {
            // Se o comando foi executado com sucesso, faça algo com o output aqui
            println!("Bot pronto para ser configurado!")
        }
        Err(e) => {
            // Se houve um erro ao executar o comando, faça algo com o erro aqui
            println!("Erro ao executar o comando: ({}) erro: {}", move_command, e);
        }
    }
}
