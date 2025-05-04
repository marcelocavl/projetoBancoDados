use crate::includes::classes::Funcionario::Funcionario;
use crate::includes::classes::Departamento::Departamento;
use crate::includes::classes::Projeto::Projeto;

use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::path::Path;

pub fn ler_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut entrada: String = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}

pub fn arquivo_existe(caminho: &str) -> bool {
    Path::new(caminho).exists()
}

pub fn criar_arquivo(caminho: &str) -> io::Result<()> {
    if !arquivo_existe(caminho) {
        println!("Arquivo não encontrado. Criando arquivo '{}'.", caminho);
        let _: File = File::create(caminho)?; // Cria o arquivo vazio
        // writeln!(file, "1")?; // ID inicial
    } else {
        println!("Arquivo '{}' já existe.", caminho);
    }
    Ok(())
}


// CRUD DE FUNCIONARIOS

// CRUD DE PROJETOS

// CRUD DE DEPARTAMENTOS
