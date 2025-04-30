mod includes;

use includes::classes::Funcionario::Funcionario;
use includes::classes::Projeto::Projeto;
use includes::Funcionario_Projeto::FuncionarioProjeto;


fn main() {
	let mut func1=Funcionario::new(
														32,
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


	ids_funcionarios.push(func1);
	ids_funcionarios.push(func2);

	ids_projetos.push(proj1);
	ids_projetos.push(proj2);

	
	let mut func_proj=FuncionarioProjeto::new(
																		ids_funcionarios,
																		ids_projetos,
																		0);

	
	let mut vetor_ids_funcs=func_proj.get_ids_funcionarios();

	vetor_ids_funcs[1].print();
}
