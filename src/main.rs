mod includes;
mod crud;
use crud::*;
use includes::classes::funcionarios_arquivo_csv::FuncionariosArquivoCSV;

use std::io::{self, Write};

fn gerenciar_funcionarios(caminho: &str) -> io::Result<()> {
    let (mut funcionarios, mut proximo_id) = carregar_funcionarios(caminho)?;
    
    loop {
        println!("\n========= FUNCIONÁRIOS =========");
        println!("1 - Listar funcionários");
        println!("2 - Criar funcionário");
        println!("3 - Editar funcionário");
        println!("4 - Remover funcionário");
        println!("0 - Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush()?;
    
        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao)?;
        let op: &str = opcao.trim();

        match op {
            "1" => {
                listar_funcionarios(&mut funcionarios)?;
            }
            "2" => {
                adicionar_funcionario_interativo(&mut funcionarios, &mut proximo_id)?;
            }
            "3" => {
                listar_funcionarios(&mut funcionarios)?;
                let id: u32 = ler_input("Digite o ID do funcionário que deseja atualizar: ")
                    .parse()
                    .unwrap_or(0);
                atualizar_funcionario_por_id(id, &mut funcionarios)?;
            }
            "4" => {
                listar_funcionarios(&mut funcionarios)?;
                let id: u32 = ler_input("Digite o ID do funcionário que deseja remover: ")
                    .parse()
                    .unwrap_or(0);
                remover_funcionario_por_id(id, &mut funcionarios)?;
            }
            "0" => {
                println!("\nSalvar as alterações?");
                println!("1 - Sim, salvar alterações");
                println!("2 - Não, descartar alterações");
                println!("0 - Cancelar");
                print!("Escolha uma opção: ");
                io::stdout().flush()?;
                
                opcao.clear();
                io::stdin().read_line(&mut opcao)?;
                let op = opcao.trim();

                match op {
                    "1" => {
                        salvar_funcionarios(caminho, &mut funcionarios, proximo_id)?;
                        let mut funcionarios_csv: FuncionariosArquivoCSV = FuncionariosArquivoCSV::new(funcionarios.clone(), "funcionarios.csv");
                        funcionarios_csv.atualizar_txt();
                        println!("Alterações salvas. Saindo...");
                        break;
                    }
                    "2" => {
                        println!("Alterações descartadas. Saindo...");
                        break;
                    }
                    "0" => {}
                    _ => println!("Opção inválida."),
                }
            }
            _ => println!("Opção inválida."),
        }
    }
    Ok(())
}

fn gerenciar_projetos(caminho: &str) -> io::Result<()> {
    let (mut projetos, mut proximo_id) = carregar_projetos(caminho)?;

    loop {
        println!("\n========= PROJETOS =========");
        println!("1 - Listar projetos");
        println!("2 - Criar projeto");
        println!("3 - Editar projeto");
        println!("4 - Remover projeto");
        println!("0 - Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush()?;

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao)?;
        let op: &str = opcao.trim();

        match op {
            "1" => {
                listar_projetos(&mut projetos)?;
            }
            "2" => {
                adicionar_projeto_interativo(&mut projetos, &mut proximo_id)?;
            }
            "3" => {
                listar_projetos(&mut projetos)?;
                let id: u32 = ler_input("Digite o ID do projeto que deseja atualizar: ")
                    .parse()
                    .unwrap_or(0);
                atualizar_projeto_por_id(id, &mut projetos)?;
            }
            "4" => {
                listar_projetos(&mut projetos)?;
                let id: u32 = ler_input("Digite o ID do projeto que deseja remover: ")
                    .parse()
                    .unwrap_or(0);
                remover_projeto_por_id(id, &mut projetos)?;
            }
            "0" => {
                println!("\nSalvar as alterações?");
                println!("1 - Sim, salvar alterações");
                println!("2 - Não, descartar alterações");
                println!("0 - Cancelar");
                print!("Escolha uma opção: ");
                io::stdout().flush()?;

                opcao.clear();
                io::stdin().read_line(&mut opcao)?;
                let op = opcao.trim();

                match op {
                    "1" => {
                        salvar_projetos(caminho, &mut projetos, proximo_id)?;
                        println!("Alterações salvas. Saindo...");
                        break;
                    }
                    "2" => {
                        println!("Alterações descartadas. Saindo...");
                        break;
                    }
                    "0" => {}
                    _ => println!("Opção inválida."),
                }
            }
            _ => println!("Opção inválida."),
        }
    }
    Ok(())
}

fn gerenciar_departamentos(caminho: &str) -> io::Result<()> {
    let (mut departamentos, mut proximo_id) = carregar_departamentos(caminho)?;

    loop {
        println!("\n========= DEPARTAMENTOS =========");
        println!("1 - Listar departamentos");
        println!("2 - Criar departamento");
        println!("3 - Editar departamento");
        println!("4 - Remover departamento");
        println!("0 - Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush()?;

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao)?;
        let op: &str = opcao.trim();

        match op {
            "1" => {
                listar_departamentos(&mut departamentos)?;
            }
            "2" => {
                adicionar_departamento_interativo(&mut departamentos, &mut proximo_id)?;
            }
            "3" => {
                listar_departamentos(&mut departamentos)?;
                let id: u32 = ler_input("Digite o ID do departamento que deseja atualizar: ")
                    .parse()
                    .unwrap_or(0);
                atualizar_departamento_por_id(id, &mut departamentos)?;
            }
            "4" => {
                listar_departamentos(&mut departamentos)?;
                let id: u32 = ler_input("Digite o ID do departamento que deseja remover: ")
                    .parse()
                    .unwrap_or(0);
                remover_departamento_por_id(id, &mut departamentos)?;
            }
            "0" => {
                println!("\nSalvar as alterações?");
                println!("1 - Sim, salvar alterações");
                println!("2 - Não, descartar alterações");
                println!("0 - Cancelar");
                print!("Escolha uma opção: ");
                io::stdout().flush()?;

                opcao.clear();
                io::stdin().read_line(&mut opcao)?;
                let op = opcao.trim();

                match op {
                    "1" => {
                        salvar_departamentos(caminho, &mut departamentos, proximo_id)?;
                        println!("Alterações salvas. Saindo...");
                        break;
                    }
                    "2" => {
                        println!("Alterações descartadas. Saindo...");
                        break;
                    }
                    "0" => {}
                    _ => println!("Opção inválida."),
                }
            }
            _ => println!("Opção inválida."),
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {

	// Criar arquivos .txt caso não existam
    let caminho_funcionarios: &str = "funcionarios.txt";
    let caminho_projetos: &str = "projetos.txt";
    let caminho_departamentos: &str = "departamentos.txt";
    let caminho_funcionarios_csv: &str = "funcionarios.csv";
    criar_arquivo(caminho_funcionarios)?;
    criar_arquivo(caminho_projetos)?;
    criar_arquivo(caminho_departamentos)?;
    criar_arquivo(caminho_funcionarios_csv)?;

	// Menu principal de gerenciamento
    loop {
        println!("\n========= GERENCIAR EMPRESA =========");
        println!("1 - Gerenciar funcionários");
		println!("2 - Gerenciar projetos");
		println!("3 - Gerenciar departamentos");
        println!("0 - Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush()?;
        
        let mut opcao: String = String::new();
        io::stdin().read_line(&mut opcao)?;
        let opcao: &str = opcao.trim();

        match opcao {
            "1" => {
                gerenciar_funcionarios(caminho_funcionarios)?;
            }
            "2" => {
                gerenciar_projetos(caminho_projetos)?;
            }
            "3" => {
                gerenciar_departamentos(caminho_departamentos)?;
            }
            "0" => {
                println!("Encerrando...");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }

    Ok(())
}
