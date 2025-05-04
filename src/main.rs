mod includes;

use includes::Funcionario_Projeto::FuncionarioProjeto;
use includes::Funcionarios::Funcionarios;
use includes::classes::Funcionario::Funcionario;
use includes::classes::Projeto::Projeto;
use includes::Menu;
use includes::Menu2;

fn main() {
    // let mut func1 = Funcionario::new(
    //     402,
    //     String::from("fulano"),
    //     String::from("123"),
    //     String::from("rua ifce"),
    //     1200.0,
    //     'm',
    //     String::from("12/2/12"),
    //     23,
    // );

    // let mut func2 = Funcionario::new(
    //     402,
    //     String::from("fulano"),
    //     String::from("123"),
    //     String::from("rua ifce"),
    //     1200.0,
    //     'm',
    //     String::from("12/2/12"),
    //     23,
    // );

    // let mut func3 = Funcionario::new(
    //     32,
    //     String::from("cicrano"),
    //     String::from("123"),
    //     String::from("rua ufc"),
    //     1200.0,
    //     'm',
    //     String::from("12/2/12"),
    //     23,
    // );

    // //comparando os dois funcionarios
    // println!("{}", func1.cmp_funcionarios(&mut func2));

    // let mut proj1 = Projeto::new(
    //     120,
    //     String::from("sistema web"),
    //     20,
    //     String::from("rua 123"),
    // );

    // let mut proj2 = Projeto::new(34, String::from("catalogo"), 49, String::from("rua ai "));

    // let mut ids_funcionarios: Vec<Funcionario> = Vec::new();
    // let mut ids_projetos: Vec<Projeto> = Vec::new();

    // let mut func_proj = FuncionarioProjeto::new(ids_funcionarios, ids_projetos, 0);

    // func_proj.adicionar_relacao(func1.clone(), proj1.clone());
    // func_proj.adicionar_relacao(func2.clone(), proj2.clone());
    // func_proj.print_ids_funcionarios_e_projetos();
    // println!("");

    // let mut lista_func: Vec<Funcionario> = Vec::new();
    // let mut tab_func: Funcionarios = Funcionarios::new(lista_func, 0);

    // tab_func.adicionar_funcionario(&func1);
    // tab_func.adicionar_funcionario(&func2);
    // tab_func.adicionar_funcionario(&func3);
    // tab_func.print_lista();
    // println!("{}", tab_func.get_num_funcionarios());

    // func1.print();
    // func1.set_id(95);
    // func1.print();
	while (true) {
		println!("Escolha uma opção:");
		println!("1 - Gerenciar Funcionários");
		println!("2 - Gerenciar Projetos");
		println!("3 - Gerenciar Funcionário-Projeto");
		println!("0 - Sair");

		let mut opcao = String::new();
		std::io::stdin().read_line(&mut opcao).unwrap();
		let opcao: u32 = opcao.trim().parse().unwrap_or(0);

		match opcao {
			1 => {
				let caminho = "./funcionarios.txt";
				Menu2::gerenciar_funcionarios(caminho).unwrap();
			}
			2 => {
				// let caminho = "src/projetos.csv";
				// Menu::gerenciar_projetos(caminho).unwrap();
			}
			3 => {
				// let caminho = "src/funcionario_projeto.csv";
				// Menu::gerenciar_funcionario_projeto(caminho).unwrap();
			}
			0 => break,
			_ => println!("Opção inválida!"),
		}
		
	}
}
