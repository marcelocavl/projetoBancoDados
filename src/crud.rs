use crate::includes::classes::funcionario::Funcionario;
use crate::includes::classes::departamento::Departamento;
use crate::includes::classes::projeto::Projeto;
use crate::includes::utils::validator::{self, Validator};




use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::path::Path;

const VALIDATOR:validator::Validator = Validator::new();

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


// CRUD DE FUNCIONARIOS

pub fn adicionar_funcionario_interativo(funcionarios: &mut Vec<Funcionario>, proximo_id: &mut u32) -> io::Result<()> {
    
    // Lê todas as linhas do arquivo
    println!("\nAdicione as informações do novo funcionário...");
    // Solicita os dados ao usuário
    let nome: String = VALIDATOR.string(
        "Nome: ",
        |input|VALIDATOR.funcionario.nome(input),
        "Nome inválido. Tente novamente.",
        ler_input,
    );

    let  cpf: String = VALIDATOR.string(
        "CPF: ",
        |input|VALIDATOR.funcionario.cpf(input),
        "CPF inválido. Tente novamente.",
        ler_input,
    );
    let endereco: String = VALIDATOR.string(
        "Endereço: ",
        |input|VALIDATOR.funcionario.endereco(input),
        "Endereço inválido. Tente novamente.",
        ler_input,
    );

    let salario: f64 = VALIDATOR.float(
        "Salário: ",
        |input|VALIDATOR.funcionario.salario(input),
        "Salário inválido. Tente novamente.",
        ler_input,
    );

    let genero: char = VALIDATOR.char(
        "Gênero(M/F): ",
        |input|VALIDATOR.funcionario.gender(input),
        "Gênero inválido. Tente novamente.",
        ler_input,
    );
    let nascimento: String = VALIDATOR.string(
        "Nascimento (YYYY-MM-DD): ",
        |input|VALIDATOR.funcionario.nascimento(input),
        "Data inválida. Tente novamente.",
        ler_input,
    );
    let id_departamento: u32 = VALIDATOR.numero(
        "ID do departamento: ",
        |input|VALIDATOR.funcionario.id_departamento(input),
        "ID inválido. Tente novamente.",
        ler_input,
    );

    let novo_funcionario: Funcionario = Funcionario::new(
        *proximo_id,
        nome.clone(),
        cpf,
        endereco,
        salario,
        genero,
        nascimento,
        id_departamento,
    );

    funcionarios.push(novo_funcionario);

    println!("Funcionário '{}' cadastrado com ID {}.", nome, *proximo_id);
    *proximo_id += 1;

    Ok(())
}

fn achar_funcionario_por_id(funcionarios: &mut Vec<Funcionario>, id: u32) -> Option<usize> {
    if funcionarios.is_empty() {
        return None; // Retorna None se a lista estiver vazia
    }
    funcionarios.iter_mut().position(|funcionario: &mut Funcionario| *funcionario.get_id() == id)
}

fn deletar_funcionario_por_id(funcionarios: &mut Vec<Funcionario>, id: u32) -> bool {
    if let Some(pos) = funcionarios.iter_mut().position(|funcionario: &mut Funcionario| *funcionario.get_id() == id) {
        funcionarios.remove(pos);
        true
    } else {
        false // Retorna false se o funcionário com o id não foi encontrado
    }
}

pub fn atualizar_funcionario_por_id(id_alvo: u32, funcionarios: &mut Vec<Funcionario>) -> io::Result<()> {
    if let Some(n) = achar_funcionario_por_id(funcionarios, id_alvo) {
        // let n = lista.achar_funcionario_por_id(id_alvo).unwrap(); // Pega o índice do funcionário
        let id: u32 = funcionarios[n].get_id().clone();
        let mut nome: String = funcionarios[n].get_nome().clone();
        let mut cpf: String = funcionarios[n].get_cpf().clone();
        let mut endereco: String = funcionarios[n].get_endereco().clone();
        let mut salario: f64 = funcionarios[n].get_salario().clone();
        let mut genero: char = funcionarios[n].get_genero().clone();
        let mut nascimento: String = funcionarios[n].get_nascimento().clone();
        let mut id_departamento: u32 = funcionarios[n].get_id_departamento().clone();

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
                "1" => {
                    funcionarios[n].set_nome(VALIDATOR.string("Novo nome: ",
                        |input|VALIDATOR.funcionario.nome(input),
                        "Nome inválido. Tente novamente.",
                        ler_input,
                    ));
                    nome = funcionarios[n].get_nome().clone(); // Atualiza o nome
                }
                "2" => {
                    funcionarios[n].set_cpf(VALIDATOR.string("Novo CPF: ",
                    |input|VALIDATOR.funcionario.cpf(input),
                    "CPF inválido. Tente novamente.",
                    ler_input,
                ));
                    cpf = funcionarios[n].get_cpf().clone(); // Atualiza o CPF
                }
                "3" => {
                    funcionarios[n].set_endereco( VALIDATOR.string("Novo endereço: ",
                        |input|VALIDATOR.funcionario.endereco(input),
                        "Endereço inválido. Tente novamente.",
                        ler_input
                    ));
                    endereco = funcionarios[n].get_endereco().clone(); // Atualiza o endereço
                }
                "4" => {
                    funcionarios[n]
                        .set_salario( VALIDATOR.float("Novo salário: ",
                            |input|VALIDATOR.funcionario.salario(input),
                            "Salário inválido. Tente novamente.",
                            ler_input
                        ));
                    salario = funcionarios[n].get_salario().clone(); // Atualiza o salário
                }
                "5" => {
                    funcionarios[n].set_genero(
                        VALIDATOR.char("Novo gênero (M/F): ",
                            |input|VALIDATOR.funcionario.gender(input),
                            "Gênero inválido. Tente novamente.",
                            ler_input)
                    );
                    genero = funcionarios[n].get_genero().clone(); // Atualiza o gênero
                }
                "6" => {
                    funcionarios[n]
                        .set_nascimento(VALIDATOR.string("Novo nascimento (YYYY-MM-DD): ",
                            |input|VALIDATOR.funcionario.nascimento(input),
                            "Data inválida. Tente novamente.",
                            ler_input,
                        ));
                    nascimento = funcionarios[n].get_nascimento().clone(); // Atualiza o nascimento
                }
                "7" => {
                    funcionarios[n].set_id_departamento(
                        VALIDATOR.numero("Novo ID do departamento: ",
                            |input|VALIDATOR.funcionario.id_departamento(input),
                            "ID inválido. Tente novamente.",
                            ler_input,
                        ));
                    
                    id_departamento = funcionarios[n].get_id_departamento().clone(); // Atualiza o ID do departamento
                }
                "0" => break,
                _ => println!("Opção inválida."),
            };
        }
    } else {
        println!("Funcionário com ID {} não encontrado.", id_alvo);
        return Ok(());
    }

    Ok(())
}

pub fn remover_funcionario_por_id(id_alvo: u32, funcionarios: &mut Vec<Funcionario>) -> io::Result<()> {
    if deletar_funcionario_por_id(funcionarios, id_alvo) {
        println!("Funcionário com ID {} removido.", id_alvo);
    } else {
        println!("Funcionário com ID {} não encontrado.", id_alvo);
        return Ok(());
    }

    Ok(())
}

pub fn listar_funcionarios(funcionarios: &mut Vec<Funcionario>) -> io::Result<()> {
    if funcionarios.is_empty() {
        println!("Nenhum funcionário cadastrado.");
        return Ok(());
    }

    println!("");
    println!(
        "{:<4} {:<25} {:<15} {:<35} {:<10} {:<6} {:<12} {:<5}",
        "| ID", "| Nome", "| CPF", "| Endereço", "| Salário", "| Gênero", "| Nascimento", "| Dep."
    );
    println!("{}", "-".repeat(120));

    for funcionario in funcionarios {
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

pub fn carregar_funcionarios(caminho: &str) -> io::Result<(Vec<Funcionario>, u32)> {
    let arquivo: File = File::open(caminho)?;
    let mut leitor: BufReader<File> = BufReader::new(arquivo);
    let mut primeira_linha: String = String::new();

    leitor.read_line(&mut primeira_linha)?;
    let proximo_id: u32 = primeira_linha.trim().parse().unwrap_or(1);

    let mut lista_funcionarios: Vec<Funcionario> = Vec::new();

    for linha in leitor.lines().flatten() {
        let campos: Vec<&str> = linha.split(';').collect();
        if campos.len() != 8 {
            continue;
        }

        let funcionario: Funcionario = Funcionario::new(
            campos[0].parse().unwrap_or(0),
            campos[1].to_string(),
            campos[2].to_string(),
            campos[3].to_string(),
            campos[4].parse().unwrap_or(0.0),
            campos[5].chars().next().unwrap_or('?'),
            campos[6].to_string(),
            campos[7].parse().unwrap_or(0),
        );

        lista_funcionarios.push(funcionario);
    }

    Ok((lista_funcionarios, proximo_id))
}

pub fn salvar_funcionarios(caminho: &str, funcionarios: &mut Vec<Funcionario>, proximo_id: u32) -> io::Result<()> {
    let mut conteudo: String = format!("{}\n", proximo_id);

    for funcionario in funcionarios {
        conteudo.push_str(&format!(
            "{};{};{};{};{};{};{};{}\n",
            funcionario.get_id().clone(),
            funcionario.get_nome().clone(),
            funcionario.get_cpf().clone(),
            funcionario.get_endereco().clone(),
            funcionario.get_salario().clone(),
            funcionario.get_genero().clone(),
            funcionario.get_nascimento().clone(),
            funcionario.get_id_departamento().clone()
        ));
    }

    std::fs::write(caminho, conteudo)?;
    Ok(())
}


// CRUD DE PROJETOS

pub fn adicionar_projeto_interativo(projetos: &mut Vec<Projeto>, proximo_id: &mut u32) -> io::Result<()> {
    println!("\nAdicione as informações do novo projeto...");
    let nome_projeto: String = VALIDATOR.string(
        "Nome do projeto: ",
        |input|VALIDATOR.projeto.nome(input),
        "Nome inválido. Tente novamente.",
        ler_input,
    );
    let id_departamento: u32 = VALIDATOR.numero(
        "ID do departamento: ",
        |input|VALIDATOR.projeto.id_departamento(input),
        "ID inválido. Tente novamente.",
        ler_input,
    );
    let local: String = VALIDATOR.string(
        "Local: ",
        |input|VALIDATOR.projeto.local(input),
        "Local inválido. Tente novamente.",
        ler_input,
    );

    let novo_projeto: Projeto = Projeto::new(*proximo_id, nome_projeto.clone(), id_departamento, local);

    projetos.push(novo_projeto);

    println!("Projeto '{}' cadastrado com ID {}.", nome_projeto, *proximo_id);
    *proximo_id += 1;

    Ok(())
}

fn achar_projeto_por_id(projetos: &mut Vec<Projeto>, id: u32) -> Option<usize> {
    if projetos.is_empty() {
        return None; // Retorna None se a lista estiver vazia
    }
    projetos.iter_mut().position(|projeto: &mut Projeto| *projeto.get_id_projeto() == id)
}

fn deletar_projeto_por_id(projetos: &mut Vec<Projeto>, id: u32) -> bool {
    if let Some(pos) = projetos.iter_mut().position(|projeto: &mut Projeto| *projeto.get_id_projeto() == id) {
        projetos.remove(pos);
        true
    } else {
        false // Retorna false se o projeto com o id não foi encontrado
    }
}

pub fn atualizar_projeto_por_id(id_alvo: u32, projetos: &mut Vec<Projeto>) -> io::Result<()> {
    if let Some(n) = achar_projeto_por_id(projetos, id_alvo) {
        let id_projeto: u32 = projetos[n].get_id_projeto().clone();
        let mut nome_projeto: String = projetos[n].get_nome_projeto().clone();
        let mut id_departamento: u32 = projetos[n].get_id_departamento().clone();
        let mut local: String = projetos[n].get_local().clone();

        println!("--- Editando projeto ID {} ---", id_projeto);
        loop {
            println!("\nCampos disponíveis para edição:");
            println!("1 - Nome do projeto: {}", nome_projeto);
            println!("2 - ID do departamento: {}", id_departamento);
            println!("3 - Local: {}", local);
            println!("0 - Finalizar edição");

            let opcao: String = ler_input("Escolha o número do campo para editar: ");

            match opcao.trim() {
                "1" => {
                    projetos[n].set_nome_projeto( 
                        VALIDATOR.string("Novo nome do projeto: ",
                            |input|VALIDATOR.projeto.nome(input),
                            "Nome inválido. Tente novamente.",
                            ler_input,
                        ));
                    nome_projeto = projetos[n].get_nome_projeto().clone();
                }
                "2" => {
                    projetos[n].set_id_departamento(
                        VALIDATOR.numero("Novo ID do departamento: ",
                            |input|VALIDATOR.projeto.id_departamento(input),
                            "ID inválido. Tente novamente.",
                            ler_input,
                        )
                    );
                    id_departamento = projetos[n].get_id_departamento().clone();
                }
                "3" => {
                    projetos[n].set_local(
                        VALIDATOR.string("Novo local: ",
                            |input|VALIDATOR.projeto.local(input),
                            "Local inválido. Tente novamente.",
                            ler_input,
                        )
                    );
                    local = projetos[n].get_local().clone();
                }
                "0" => break,
                _ => println!("Opção inválida."),
            };
        }
    } else {
        println!("Projeto com ID {} não encontrado.", id_alvo);
        return Ok(());
    }

    Ok(())
}

pub fn remover_projeto_por_id(id_alvo: u32, projetos: &mut Vec<Projeto>) -> io::Result<()> {
    if deletar_projeto_por_id(projetos, id_alvo) {
        println!("Projeto com ID {} removido.", id_alvo);
    } else {
        println!("Projeto com ID {} não encontrado.", id_alvo);
        return Ok(());
    }

    Ok(())
}

pub fn listar_projetos(projetos: &mut Vec<Projeto>) -> io::Result<()> {
    if projetos.is_empty() {
        println!("Nenhum projeto cadastrado.");
        return Ok(());
    }

    println!("");
    println!(
        "{:<4} {:<25} {:<15} {:<35}",
        "| ID", "| Nome do Projeto", "| ID Departamento", "| Local"
    );
    println!("{}", "-".repeat(80));

    for projeto in projetos {
        println!(
            "| {:<3}| {:<24}| {:<14}| {:<34}",
            projeto.get_id_projeto().clone(),
            projeto.get_nome_projeto().clone(),
            projeto.get_id_departamento().clone(),
            projeto.get_local().clone()
        );
    }

    Ok(())
}

pub fn carregar_projetos(caminho: &str) -> io::Result<(Vec<Projeto>, u32)> {
    let arquivo: File = File::open(caminho)?;
    let mut leitor: BufReader<File> = BufReader::new(arquivo);
    let mut primeira_linha: String = String::new();

    leitor.read_line(&mut primeira_linha)?;
    let proximo_id: u32 = primeira_linha.trim().parse().unwrap_or(1);

    let mut lista_projetos: Vec<Projeto> = Vec::new();

    for linha in leitor.lines().flatten() {
        let campos: Vec<&str> = linha.split(';').collect();
        if campos.len() != 4 {
            continue;
        }

        let projeto: Projeto = Projeto::new(
            campos[0].parse().unwrap_or(0),
            campos[1].to_string(),
            campos[2].parse().unwrap_or(0),
            campos[3].to_string(),
        );

        lista_projetos.push(projeto);
    }

    Ok((lista_projetos, proximo_id))
}

pub fn salvar_projetos(caminho: &str, projetos: &mut Vec<Projeto>, proximo_id: u32) -> io::Result<()> {
    let mut conteudo: String = format!("{}\n", proximo_id);

    for projeto in projetos {
        conteudo.push_str(&format!(
            "{};{};{};{}\n",
            projeto.get_id_projeto().clone(),
            projeto.get_nome_projeto().clone(),
            projeto.get_id_departamento().clone(),
            projeto.get_local().clone()
        ));
    }

    std::fs::write(caminho, conteudo)?;
    Ok(())
}


// CRUD DE DEPARTAMENTOS

pub fn adicionar_departamento_interativo(departamentos: &mut Vec<Departamento>, proximo_id: &mut u32) -> io::Result<()> {
    println!("\nAdicione as informações do novo departamento...");
    let nome_departamento: String = VALIDATOR.string(
        "Nome do departamento: ",
        |input|VALIDATOR.departamento.nome(input),
        "Nome inválido. Tente novamente.",
        ler_input,
    );
    let id_gerente: u32 = VALIDATOR.numero(
        "ID do gerente: ",
        |input|VALIDATOR.departamento.id_gerente(input),
        "ID inválido. Tente novamente.",
        ler_input,
    );

    let novo_departamento: Departamento = Departamento::new(*proximo_id, nome_departamento.clone(), id_gerente);

    departamentos.push(novo_departamento);

    println!("Departamento '{}' cadastrado com ID {}.", nome_departamento, *proximo_id);
    *proximo_id += 1;

    Ok(())
}

fn achar_departamento_por_id(departamentos: &mut Vec<Departamento>, id: u32) -> Option<usize> {
    if departamentos.is_empty() {
        return None; // Retorna None se a lista estiver vazia
    }
    departamentos.iter_mut().position(|departamento: &mut Departamento| *departamento.get_id() == id)
}

fn deletar_departamento_por_id(departamentos: &mut Vec<Departamento>, id: u32) -> bool {
    if let Some(pos) = departamentos.iter_mut().position(|departamento: &mut Departamento| *departamento.get_id() == id) {
        departamentos.remove(pos);
        true
    } else {
        false // Retorna false se o departamento com o id não foi encontrado
    }
}

pub fn atualizar_departamento_por_id(id_alvo: u32, departamentos: &mut Vec<Departamento>) -> io::Result<()> {
    if let Some(n) = achar_departamento_por_id(departamentos, id_alvo) {
        let id_departamento: u32 = departamentos[n].get_id().clone();
        let mut nome_departamento: String = departamentos[n].get_nome().clone();
        let mut id_gerente: u32 = departamentos[n].get_id_gerente().clone();

        println!("--- Editando departamento ID {} ---", id_departamento);
        loop {
            println!("\nCampos disponíveis para edição:");
            println!("1 - Nome do departamento: {}", nome_departamento);
            println!("2 - ID do gerente: {}", id_gerente);
            println!("0 - Finalizar edição");

            let opcao = ler_input("Escolha o número do campo para editar: ");

            match opcao.trim() {
                "1" => {
                    departamentos[n].set_nome(
                        VALIDATOR.string("Novo nome do departamento: ",
                            |input|VALIDATOR.departamento.nome(input),
                            "Nome inválido. Tente novamente.",
                            ler_input,
                        ));
                    nome_departamento = departamentos[n].get_nome().clone();
                }
                "2" => {
                    departamentos[n].set_id_gerente(
                        VALIDATOR.numero("Novo ID do gerente: ",
                            |input|VALIDATOR.departamento.id_gerente(input),
                            "ID inválido. Tente novamente.",
                            ler_input,
                        )
                    );
                    id_gerente = departamentos[n].get_id_gerente().clone();
                }
                "0" => break,
                _ => println!("Opção inválida."),
            };
        }
    } else {
        println!("Departamento com ID {} não encontrado.", id_alvo);
        return Ok(());
    }

    Ok(())
}

pub fn remover_departamento_por_id(id_alvo: u32, departamentos: &mut Vec<Departamento>) -> io::Result<()> {
    if deletar_departamento_por_id(departamentos, id_alvo) {
        println!("Departamento com ID {} removido.", id_alvo);
    } else {
        println!("Departamento com ID {} não encontrado.", id_alvo);
        return Ok(());
    }

    Ok(())
}

pub fn listar_departamentos(departamentos: &mut Vec<Departamento>) -> io::Result<()> {
    if departamentos.is_empty() {
        println!("Nenhum departamento cadastrado.");
        return Ok(());
    }

    println!("");
    println!(
        "{:<4} {:<25} {:<15}",
        "| ID", "| Nome do Departamento", "| ID Gerente"
    );
    println!("{}", "-".repeat(60));

    for departamento in departamentos {
        println!(
            "| {:<3}| {:<24}| {:<14}",
            departamento.get_id().clone(),
            departamento.get_nome().clone(),
            departamento.get_id_gerente().clone()
        );
    }

    Ok(())
}

pub fn carregar_departamentos(caminho: &str) -> io::Result<(Vec<Departamento>, u32)> {
    let arquivo: File = File::open(caminho)?;
    let mut leitor: BufReader<File> = BufReader::new(arquivo);
    let mut primeira_linha: String = String::new();

    leitor.read_line(&mut primeira_linha)?;
    let proximo_id: u32 = primeira_linha.trim().parse().unwrap_or(1);

    let mut lista_departamentos: Vec<Departamento> = Vec::new();

    for linha in leitor.lines().flatten() {
        let campos: Vec<&str> = linha.split(';').collect();
        if campos.len() != 3 {
            continue;
        }

        let departamento: Departamento = Departamento::new(
            campos[0].parse().unwrap_or(0),
            campos[1].to_string(),
            campos[2].parse().unwrap_or(0),
        );

        lista_departamentos.push(departamento);
    }

    Ok((lista_departamentos, proximo_id))
}

pub fn salvar_departamentos(caminho: &str, departamentos: &mut Vec<Departamento>, proximo_id: u32) -> io::Result<()> {
    let mut conteudo: String = format!("{}\n", proximo_id);

    for departamento in departamentos {
        conteudo.push_str(&format!(
            "{};{};{}\n",
            departamento.get_id().clone(),
            departamento.get_nome().clone(),
            departamento.get_id_gerente().clone()
        ));
    }

    std::fs::write(caminho, conteudo)?;
    Ok(())
}
