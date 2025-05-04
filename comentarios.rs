// MAIN

/*
        let mut func1=Funcionario::new(
                                                            56,
                                                            String::from("cicrano"),
                                                            String::from("456"),
                                                            String::from("rua slaa"),
                                                            4000.0,
                                                            'm',
                                                            String::from("12/22/89")
                                                            ,34);

        let mut func2=Funcionario::new(
                                                            402,
                                                            String::from("fulano"),
                                                            String::from("123"),
                                                            String::from("rua ifce"),
                                                            1200.0,
                                                            'm',
                                                            String::from("12/2/12")
                                                            ,23);

      //inicializando lista funcionarios
      let mut vec_func:Vec<Funcionario>=Vec::new();
      let mut lista_func:Funcionarios=Funcionarios::new(vec_func,0);


      let mut arquivo_func:funcionarios_arquivotxt=funcionarios_arquivotxt::new(lista_func,path);

      arquivo_func.get_funcionarios().adicionar_funcionario(func1);
      arquivo_func.get_funcionarios().adicionar_funcionario(func2);

      arquivo_func.atualizar_txt();

        let mut func3=Funcionario::new(
                                                            100,
                                                            String::from("beltrano"),
                                                            String::from("131131"),
                                                            String::from("rua ufc"),
                                                            120123.0,
                                                            'm',
                                                            String::from("12/2/12")
                                                            ,123123);



      arquivo_func.get_funcionarios().adicionar_funcionario(func3);



      arquivo_func.atualizar_txt();
    */
    /*
    //comparando os dois funcionarios
        println!("{}",func1.cmp_funcionarios(&mut func2));


        let mut proj1=Projeto::new(
                                                     120,
                                                     String::from("sistema web"),
                                                     20,
                                                     String::from("rua 123"));


        let mut proj2=Projeto::new(
                                                     34,
                                                     String::from("catalogo"),
                                                     49,
                                                     String::from("rua ai "));

        let mut ids_funcionarios:Vec<Funcionario>=Vec::new();
        let mut ids_projetos:Vec<Projeto>=Vec::new();


        let mut func_proj=FuncionarioProjeto::new(
                                                                            ids_funcionarios,
                                                                            ids_projetos,
                                                                            0);

        func_proj.adicionar_relacao(func1.clone(),proj1.clone());
        func_proj.adicionar_relacao(func2.clone(),proj2.clone());
        func_proj.print_ids_funcionarios_e_projetos();
        println!("");

        let mut lista_func:Vec<Funcionario>=Vec::new();
        let mut tab_func:Funcionarios=Funcionarios::new(
                                                                                                lista_func,
                                                                                                0);

        tab_func.adicionar_funcionario(func1);
        tab_func.adicionar_funcionario(func2);
        tab_func.adicionar_funcionario(func3);
        tab_func.print_lista();
        println!("{}",tab_func.get_num_funcionarios());
    */

// MENU
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

//DEPARTAMENTO
//funcao retorna id do departamento
//funcao retorna nome do departamento
//funcao retorna id do gerente
//funcao altera o id do departamento
//funcao altera o nome do departamento
//funcao altera o id do gerente
//funcao imprime os dados do departamento

//FUNCIONARIO
//funcao retorna id do funcionario
//funcao retorna nome do funcionario
//funcao retorna cpf do funcionario
//funcao retorna id do funcionario
//funcao retorna id do funcionario
//funcao retorna id do funcionario
//funcao retorna id do funcionario
//funcao retorna id do funcionario
//funcao retorna id do funcionario
//funcao retorna nome do funcionario
//funcao retorna cpf do funcionario
//funcao retorna id do funcionario
//funcao retorna id do funcionario
//funcao retorna id do funcionario
//funcao retorna id do funcionario
//funcao retorna id do funcionario
//funcao para printar o usuario
//funcao para printar id funcionario
//funcao para printar nome funcionario
//funcao para printar cpf usuario
//funcao para printar endereco usuario
//funcao para printar genero usuario
//funcao para printar salario usuario
//funcao para printar id do departamento do usuario
//funcao para printar nascimento usuario
// Função de comparar dois funcionarios

// PROJETO
 //funcao retorna id do projeto
//funcao retorna nome do projeto
//funcao retorna id do departamento associado
//funcao retorna local do projeto
//funcao altera id do projeto
//funcao altera nome do projeto
//funcao altera id do departamento associado
//funcao altera local do projeto
//funcao imprime os dados do projeto
//funcao para printar id do projeto
//funcao para printar nome do projeto
//funcao para printar local
//funcao para printar id do departamento


// main antiga
//mod includes;
// mod menu;

// // use includes::Funcionario_Projeto::FuncionarioProjeto;
// // use includes::Funcionarios::Funcionarios;
// // use menu::*;
// use includes::classes::Funcionario::Funcionario;
// use includes::classes::Departamento::Departamento;
// use includes::classes::Projeto::Projeto;
// // use includes::editores_txt::funcionarios_arquivotxt::funcionarios_arquivotxt;

// fn main() {
//     let funcionarios: Vec<Funcionario> = Vec::new();
//     // let lista_func: Funcionarios = Funcionarios::new(vec_func, 0);
//     let path_funcionarios: &str = "./src/arquivos_txt/funcionarios.txt";

//     // let mut func_arquivotxt = funcionarios_arquivotxt::new(lista_func, path);

//     while true {
//         // criar_funcionario(&mut func_arquivotxt);
//     }
// }