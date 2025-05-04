
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::path::Path;
use crate::includes::classes::Funcionario::Funcionario;
use crate::includes::editores_txt::funcionarios_arquivotxt::funcionarios_arquivotxt;

fn ler_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut entrada: String = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}
/*
fn arquivo_existe(caminho: &str) -> bool {
    Path::new(caminho).exists()
}

fn criar_arquivo(caminho: &str) -> io::Result<()> {
    if !arquivo_existe(caminho) {
        println!("Arquivo não encontrado. Criando arquivo '{}'.", caminho);
        let mut file: File = File::create(caminho)?; // Cria o arquivo vazio
        writeln!(file, "1")?; // ID inicial
    } else {
        println!("Arquivo '{}' já existe.", caminho);
    }
    Ok(())
}
*/
// Operações com funcionários

    pub fn criar_funcionario(funcionarios_filetxt:&mut funcionarios_arquivotxt) {
    println!("\nAdicione as informações do novo funcionário...");

    let id: u32 = ler_input("ID: ").parse().unwrap_or(0);
    let nome = ler_input("Nome: ");
    let cpf = ler_input("CPF: ");
    let endereco = ler_input("Endereço: ");
    let salario: f64 = ler_input("Salário: ").parse().unwrap_or(0.0);
    let genero: char = ler_input("Gênero (M/F): ").chars().next().unwrap_or('?');
    let nascimento = ler_input("Nascimento (YYYY-MM-DD): ");
    let id_departamento: u32 = ler_input("ID do departamento: ").parse().unwrap_or(0);

    let novo_funcionario:Funcionario=Funcionario::new(
                                                  id,
                                                  nome,
                                                  cpf,
                                                  endereco,
                                                  salario,
                                                  genero,
                                                  nascimento,
                                                  id_departamento
    );

    funcionarios_filetxt.adicionar_funcionario(novo_funcionario);
    //*proximo_id += 1;

    println!("Funcionário criado com sucesso!");
}
    

