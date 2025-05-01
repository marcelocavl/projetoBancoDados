mod includes;

use includes::classes::Funcionario::Funcionario;
use includes::classes::Projeto::Projeto;
use includes::Funcionario_Projeto::FuncionarioProjeto;


fn main() {
	let mut func1=Funcionario::new(
														402,
														String::from("fulano"),
														String::from("123"),
														String::from("rua ifce"),
														1200.0,
														'm',
														String::from("12/2/12")
														,23);

	let mut func2=Funcionario::new(
														32,
														String::from("cicrano"),
														String::from("123"),
														String::from("rua ufc"),
														1200.0,
														'm',
														String::from("12/2/12")
														,23);

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

	func_proj.adicionar_relacao(func1,proj1);
	func_proj.adicionar_relacao(func2,proj2);

	func_proj.print_ids_funcionarios_e_projetos();


}
