use super::Funcionario_Projeto::FuncionarioProjeto;
use super::Funcionarios::Funcionarios;
use super::classes::Funcionario::Funcionario;
use super::classes::Projeto::Projeto;
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
        let mut file: File = File::create(caminho)?; // Cria o arquivo vazio
        writeln!(file, "1")?; // ID inicial
    } else {
        println!("Arquivo '{}' já existe.", caminho);
    }
    Ok(())
}

pub fn adicionar_funcionario_interativo(caminho: &str, lista: &mut Funcionarios) -> io::Result<()> {
    // Lê todas as linhas do arquivo
    let mut linhas: Vec<String> = std::fs::read_to_string(caminho)?
        .lines()
        .map(|l| l.to_string())
        .collect();

    // Pega e incrementa o ID
    let id_atual = linhas
        .get(0)
        .and_then(|linha| linha.parse::<u32>().ok())
        .unwrap_or(1);
    let novo_id = id_atual;

    // Solicita os dados ao usuário
    let nome = ler_input("Nome: ");
    let cpf = ler_input("CPF: ");
    let endereco = ler_input("Endereço: ");
    let salario = ler_input("Salário: ").parse().unwrap_or(0.0);
    let genero = ler_input("Gênero (M/F): ").chars().next().unwrap_or('?');
    let nascimento = ler_input("Nascimento (YYYY-MM-DD): ");
    let id_departamento = ler_input("ID do departamento: ").parse().unwrap_or(0);

    let novo_funcionario = Funcionario::new(
        novo_id,
        nome.clone(),
        cpf.clone(),
        endereco.clone(),
        salario,
        genero,
        nascimento.clone(),
        id_departamento,
    );
    lista.adicionar_funcionario(novo_funcionario);


    let nova_linha = format!(
        "{};{};{};{};{};{};{};{}",
        novo_id, nome, cpf, endereco, salario, genero, nascimento, id_departamento
    );

    // Atualiza a primeira linha com o novo ID (incrementado)
    linhas[0] = (novo_id + 1).to_string();
    linhas.push(nova_linha);

    // Escreve tudo de volta ao arquivo
    std::fs::write(caminho, linhas.join("\n") + "\n")?;

    println!("Funcionário '{}' cadastrado com ID {}.", nome, novo_id);
    Ok(())
}

fn atualizar_funcionario_por_id(caminho: &str, id_alvo: u32, lista: &mut Funcionarios) -> io::Result<()> {
    let linhas: Vec<String> = std::fs::read_to_string(caminho)?
        .lines()
        .map(|l| l.to_string())
        .collect();

    if linhas.len() < 2 {
        println!("Nenhum funcionário cadastrado.");
        return Ok(());
    }

    let mut novas_linhas = vec![linhas[0].clone()]; // preserva ID máximo
    let mut encontrado = false;

    for linha in &linhas[1..] {
        let campos: Vec<&str> = linha.split(';').collect();

        if campos.len() != 8 {
            novas_linhas.push(linha.clone());
            continue;
        }

        let id: u32 = campos[0].parse().unwrap_or(0);

        if id == id_alvo {
            let mut n = lista.achar_funcionario_por_id(id_alvo).unwrap_or(0);
            
            let mut id = lista.get_lista()[n].get_id().clone();
            let mut nome = lista.get_lista()[n].get_nome().clone();
            let mut cpf = lista.get_lista()[n].get_cpf().clone();
            let mut endereco = lista.get_lista()[n].get_endereco().clone();
            let mut salario = lista.get_lista()[n].get_salario().clone();
            let mut genero = lista.get_lista()[n].get_genero().clone();
            let mut nascimento =lista.get_lista()[n].get_nascimento().clone();
            let mut id_departamento = lista.get_lista()[n].get_id_departamento().clone();

            println!("--- Editando funcionário ID {} ---", id);
            loop {
                println!("\nCampos disponíveis para edição:");
                println!("1 - Nome: {}", nome);
                println!("2 - CPF: {}", cpf);
                println!("3 - Endereço: {}", endereco);
                println!("4 - Salário: {:.2}", salario);
                println!("5 - Gênero: {}", genero);
                println!("6 - Nascimento: {}", nascimento);
                println!("7 - ID Departamento: {}", id_departamento);
                println!("0 - Finalizar edição");

                let opcao = ler_input("Escolha o número do campo para editar: ");

                match opcao.trim() {
                    "1" => {lista.get_lista()[n].set_nome(ler_input("Novo nome: "));
                        nome = lista.get_lista()[n].get_nome().clone(); // Atualiza o nome
                    },
                    "2" => {lista.get_lista()[n].set_cpf(ler_input("Novo CPF: "));
                        cpf = lista.get_lista()[n].get_cpf().clone(); // Atualiza o CPF
                    },
                    "3" => {lista.get_lista()[n].set_endereco(ler_input("Novo endereço: "));
                        endereco = lista.get_lista()[n].get_endereco().clone(); // Atualiza o endereço
                    },
                    "4" => {lista.get_lista()[n].set_salario(ler_input("Novo salário: ").parse().unwrap_or(salario));
                        salario = lista.get_lista()[n].get_salario().clone(); // Atualiza o salário
                    },
                    "5" => {lista.get_lista()[n].set_genero(ler_input("Novo gênero (M/F): ").chars().next().unwrap_or(genero));
                        genero = lista.get_lista()[n].get_genero().clone(); // Atualiza o gênero
                    },
                    "6" => {lista.get_lista()[n].set_nascimento(ler_input("Novo nascimento (YYYY-MM-DD): "));
                        nascimento = lista.get_lista()[n].get_nascimento().clone(); // Atualiza o nascimento
                    },
                    "7" => {lista.get_lista()[n].set_id_departamento(ler_input("Novo ID do departamento: ").parse().unwrap_or(id_departamento));
                        id_departamento = lista.get_lista()[n].get_id_departamento().clone(); // Atualiza o ID do departamento
                    },
                    "0" => break,
                    _ => println!("Opção inválida."),
                };
            }

            let nova_linha = format!(
                "{};{};{};{};{};{};{};{}",
                id,
                nome,
                cpf,
                endereco,
                salario,
                genero,
                nascimento,
                id_departamento
            );

            novas_linhas.push(nova_linha);
            encontrado = true;
        } else {
            novas_linhas.push(linha.clone());
        }
    }

    if encontrado {
        std::fs::write(caminho, novas_linhas.join("\n") + "\n")?;
        println!("Funcionário ID {} atualizado com sucesso!", id_alvo);
    } else {
        println!("Funcionário com ID {} não encontrado.", id_alvo);
    }

    Ok(())
}

fn remover_funcionario_por_id(caminho: &str, id_alvo: u32, lista: &mut Funcionarios) -> io::Result<()> {
    let linhas: Vec<String> = std::fs::read_to_string(caminho)?
        .lines()
        .map(|l: &str| l.to_string())
        .collect();

    if linhas.len() < 2 {
        println!("Nenhum funcionário cadastrado.");
        return Ok(());
    }

    let mut novas_linhas = vec![linhas[0].clone()]; // mantém o ID máximo
    let mut removido = false;



    for linha in &linhas[1..] {
        let campos: Vec<&str> = linha.split(';').collect();

        if campos.len() != 8 {
            novas_linhas.push(linha.clone());
            continue;
        }

        let id: u32 = campos[0].parse().unwrap_or(0);

        if id == id_alvo {
            removido = true;
            lista.deletar_funcionario_por_id(id_alvo);
        } else {
            novas_linhas.push(linha.clone());
        }
    }

    if removido {
        std::fs::write(caminho, novas_linhas.join("\n") + "\n")?;
        println!("Funcionário ID {} removido com sucesso!", id_alvo);
    } else {
        println!("Funcionário com ID {} não encontrado.", id_alvo);
    }

    Ok(())
}

pub fn listar_funcionarios(caminho: &str) -> io::Result<()> {
    let arquivo: File = File::open(caminho)?;
    let leitor: BufReader<File> = BufReader::new(arquivo);

    let linhas: std::iter::Skip<io::Lines<BufReader<File>>> = leitor.lines().skip(1); // pula a linha do ID

    println!("");
    println!(
        "{:<4} {:<25} {:<15} {:<35} {:<10} {:<6} {:<12} {:<5}",
        "| ID", "| Nome", "| CPF", "| Endereço", "| Salário", "| Gênero", "| Nascimento", "| Dep."
    );
    println!("{}", "-".repeat(120));

    for (i, linha) in linhas.enumerate() {
        let linha: String = linha?;
        let campos: Vec<&str> = linha.split(';').collect();

        if campos.len() != 8 {
            eprintln!("Linha {} inválida: {}", i + 2, linha); // +2 por causa do skip(1) e index 0
            continue;
        }

        let mut funcionario: Funcionario = Funcionario::new(
            campos[0].parse().unwrap_or(0),
            campos[1].to_string(),
            campos[2].to_string(),
            campos[3].to_string(),
            campos[4].parse().unwrap_or(0.0),
            campos[5].chars().next().unwrap_or('?'),
            campos[6].to_string(),
            campos[7].parse().unwrap_or(0),
        );

        println!(
            "| {:<3}| {:<24}| {:<14}| {:<34}| {:<9.2}| {:<7}| {:<11}| {:<5}",
            funcionario.get_id().clone(),
            funcionario.get_nome().clone(),
            funcionario.get_cpf().clone(),
            funcionario.get_endereco().clone(),
            funcionario.get_salario().clone(),
            funcionario.get_genero().clone(),
            funcionario.get_nascimento().clone(),
            funcionario.get_id_departamento().clone()
        );
    }
    Ok(())
}

pub fn gerenciar_funcionarios(caminho: &str, mut lista: Funcionarios) -> io::Result<()> {
    criar_arquivo(caminho)?;
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
        let opcao: &str = opcao.trim();

        match opcao {
            "1" => {
                listar_funcionarios(caminho)?;
            }
            "2" => {
                adicionar_funcionario_interativo(caminho, &mut lista)?;
            }
            "3" => {
                listar_funcionarios(caminho)?;
                let id: u32 = ler_input("Digite o ID do funcionário que deseja atualizar: ")
                    .parse()
                    .unwrap_or(0);
                atualizar_funcionario_por_id(caminho, id, &mut lista)?;
            }
            "4" => {
                listar_funcionarios(caminho)?;
                let id: u32 = ler_input("Digite o ID do funcionário que deseja remover: ")
                    .parse()
                    .unwrap_or(0);
                remover_funcionario_por_id(caminho, id, &mut lista)?;
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