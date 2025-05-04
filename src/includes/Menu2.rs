use super::Funcionario_Projeto::FuncionarioProjeto;
use super::Funcionarios::Funcionarios;
use super::classes::Funcionario::Funcionario;
use super::classes::Projeto::Projeto;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
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

pub fn adicionar_funcionario_interativo(
    lista: &mut Funcionarios,
    proximo_id: &mut u32,
) -> io::Result<()> {
    // Lê todas as linhas do arquivo
    println!("\nAdicione as informações do novo funcionário...");
    // Solicita os dados ao usuário
    let nome = ler_input("Nome: ");
    let cpf = ler_input("CPF: ");
    let endereco = ler_input("Endereço: ");
    let salario = ler_input("Salário: ").parse().unwrap_or(0.0);
    let genero = ler_input("Gênero (M/F): ").chars().next().unwrap_or('?');
    let nascimento = ler_input("Nascimento (YYYY-MM-DD): ");
    let id_departamento = ler_input("ID do departamento: ").parse().unwrap_or(0);

    let novo_funcionario = Funcionario::new(
        *proximo_id,
        nome.clone(),
        cpf,
        endereco,
        salario,
        genero,
        nascimento,
        id_departamento,
    );

    lista.adicionar_funcionario(novo_funcionario);

    println!("Funcionário '{}' cadastrado com ID {}.", nome, *proximo_id);
    *proximo_id += 1;

    Ok(())
}

fn atualizar_funcionario_por_id(id_alvo: u32, lista: &mut Funcionarios) -> io::Result<()> {
    if let Some(n) = lista.achar_funcionario_por_id(id_alvo) {
        // let n = lista.achar_funcionario_por_id(id_alvo).unwrap(); // Pega o índice do funcionário
        let id = lista.get_lista()[n].get_id().clone();
        let mut nome = lista.get_lista()[n].get_nome().clone();
        let mut cpf = lista.get_lista()[n].get_cpf().clone();
        let mut endereco = lista.get_lista()[n].get_endereco().clone();
        let mut salario = lista.get_lista()[n].get_salario().clone();
        let mut genero = lista.get_lista()[n].get_genero().clone();
        let mut nascimento = lista.get_lista()[n].get_nascimento().clone();
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
                "1" => {
                    lista.get_lista()[n].set_nome(ler_input("Novo nome: "));
                    nome = lista.get_lista()[n].get_nome().clone(); // Atualiza o nome
                }
                "2" => {
                    lista.get_lista()[n].set_cpf(ler_input("Novo CPF: "));
                    cpf = lista.get_lista()[n].get_cpf().clone(); // Atualiza o CPF
                }
                "3" => {
                    lista.get_lista()[n].set_endereco(ler_input("Novo endereço: "));
                    endereco = lista.get_lista()[n].get_endereco().clone(); // Atualiza o endereço
                }
                "4" => {
                    lista.get_lista()[n]
                        .set_salario(ler_input("Novo salário: ").parse().unwrap_or(salario));
                    salario = lista.get_lista()[n].get_salario().clone(); // Atualiza o salário
                }
                "5" => {
                    lista.get_lista()[n].set_genero(
                        ler_input("Novo gênero (M/F): ")
                            .chars()
                            .next()
                            .unwrap_or(genero),
                    );
                    genero = lista.get_lista()[n].get_genero().clone(); // Atualiza o gênero
                }
                "6" => {
                    lista.get_lista()[n]
                        .set_nascimento(ler_input("Novo nascimento (YYYY-MM-DD): "));
                    nascimento = lista.get_lista()[n].get_nascimento().clone(); // Atualiza o nascimento
                }
                "7" => {
                    lista.get_lista()[n].set_id_departamento(
                        ler_input("Novo ID do departamento: ")
                            .parse()
                            .unwrap_or(id_departamento),
                    );
                    id_departamento = lista.get_lista()[n].get_id_departamento().clone(); // Atualiza o ID do departamento
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

fn remover_funcionario_por_id(id_alvo: u32, lista: &mut Funcionarios) -> io::Result<()> {
    if lista.deletar_funcionario_por_id(id_alvo) {
        println!("Funcionário com ID {} removido.", id_alvo);
    } else {
        println!("Funcionário com ID {} não encontrado.", id_alvo);
        return Ok(());
    }

    Ok(())
}

pub fn listar_funcionarios(lista: &mut Funcionarios) -> io::Result<()> {
    if lista.get_lista().is_empty() {
        println!("Nenhum funcionário cadastrado.");
        return Ok(());
    }

    println!("");
    println!(
        "{:<4} {:<25} {:<15} {:<35} {:<10} {:<6} {:<12} {:<5}",
        "| ID", "| Nome", "| CPF", "| Endereço", "| Salário", "| Gênero", "| Nascimento", "| Dep."
    );
    println!("{}", "-".repeat(120));

    for funcionario in lista.get_lista() {
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

fn carregar_funcionarios(caminho: &str) -> io::Result<(Funcionarios, u32)> {
    let arquivo = File::open(caminho)?;
    let mut leitor = BufReader::new(arquivo);
    let mut primeira_linha = String::new();

    leitor.read_line(&mut primeira_linha)?;
    let proximo_id: u32 = primeira_linha.trim().parse().unwrap_or(1);

    let mut lista_funcionarios = Funcionarios::new(Vec::new(), 0);

    for linha in leitor.lines().flatten() {
        let campos: Vec<&str> = linha.split(';').collect();
        if campos.len() != 8 {
            continue;
        }

        let funcionario = Funcionario::new(
            campos[0].parse().unwrap_or(0),
            campos[1].to_string(),
            campos[2].to_string(),
            campos[3].to_string(),
            campos[4].parse().unwrap_or(0.0),
            campos[5].chars().next().unwrap_or('?'),
            campos[6].to_string(),
            campos[7].parse().unwrap_or(0),
        );

        lista_funcionarios.adicionar_funcionario(funcionario);
    }

    Ok((lista_funcionarios, proximo_id))
}

fn salvar_funcionarios(caminho: &str, lista: &mut Funcionarios, proximo_id: u32) -> io::Result<()> {
    let mut conteudo = format!("{}\n", proximo_id);

    for funcionario in lista.get_lista() {
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

pub fn gerenciar_funcionarios(caminho: &str) -> io::Result<()> {
    criar_arquivo(caminho)?;
    let (mut lista, mut proximo_id) = carregar_funcionarios(caminho)?;

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
                listar_funcionarios(&mut lista)?;
            }
            "2" => {
                adicionar_funcionario_interativo(&mut lista, &mut proximo_id)?;
            }
            "3" => {
                listar_funcionarios(&mut lista)?;
                let id: u32 = ler_input("Digite o ID do funcionário que deseja atualizar: ")
                    .parse()
                    .unwrap_or(0);
                atualizar_funcionario_por_id(id, &mut lista)?;
            }
            "4" => {
                listar_funcionarios(&mut lista)?;
                let id: u32 = ler_input("Digite o ID do funcionário que deseja remover: ")
                    .parse()
                    .unwrap_or(0);
                remover_funcionario_por_id(id, &mut lista)?;
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
                        salvar_funcionarios(caminho, &mut lista, proximo_id)?;
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
