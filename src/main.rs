mod dwn_repo;
mod unzip;
use dwn_repo::download_repository;
use open;
use std::env::consts::OS;
use std::io::Error;
use std::process::Command;
use std::process::Output;

fn main() {
    let verify_python_commands: Vec<&str> = vec!["python", "python3", "py"];
    let verify_pip_commands: Vec<&str> = vec!["pip", "pip3"];
    let mut python_command: &str = "";
    let mut pip_command: &str = "";
    let mut python_installed: bool = false;
    let mut pip_installed: bool = false;
    let os_name = OS;

    for command in verify_python_commands {
        
        let verify_python_installed: Result<Output, Error> =
            Command::new(command).arg("--version").output();

        match verify_python_installed {
            Ok(output) => {
                
                python_command = command;
                python_installed = true;
                print!(
                    "Python version: {}",
                    String::from_utf8_lossy(&output.stdout)
                );
                break;
            }
            Err(e) => {
                
                println!("Erro ao executar o comando: ({}) erro: {}", command, e);
            }
        }
    }

    for command in verify_pip_commands {
        
        let verify_pip_installed: Result<Output, Error> =
            Command::new(command).arg("--version").output();

        match verify_pip_installed {
            Ok(output) => {
                
                pip_command = command;
                pip_installed = true;
                print!("Pip version: {}", String::from_utf8_lossy(&output.stdout));
                break;
            }
            Err(e) => {
                
                println!("Erro ao executar o comando: ({}) erro: {}", command, e);
            }
        }
    }

    if !python_installed {
        println!("Python não está instalado no sistema");
        open::that("https://www.python.org/downloads/").expect("Erro ao abrir o navegador");
        print!("Por favor, instale o Python e tente novamente!");
        std::process::exit(0);
    } else {
        if !pip_installed {
            println!("Vamos tentar instalar o pip para você");
            let install_pip: Result<Output, Error> = Command::new(python_command)
                .arg("-m")
                .arg("ensurepip")
                .arg("--default-pip")
                .output();
            match install_pip {
                Ok(output) => {
                    
                    print!("Result: {}", String::from_utf8_lossy(&output.stdout));
                    println!("Pip instalado com sucesso!\nPor favor, execute o programa novamente");
                    std::process::exit(0);
                }
                Err(e) => {
                    
                    println!("Erro ao executar o comando: ({}) erro: {}", pip_command, e);
                    std::process::exit(0);
                }
            }
        }
        println!(
            "=======================\nPython command: {} \nPip command: {}\nOS name: {:?}\n=======================",
            python_command, pip_command, os_name
        );
    }
    let url = "https://github.com/misterioso013/kakashi-tg-bot/archive/refs/heads/main.zip";
    download_repository(url, "kakashi-tg-bot", "main");
}
