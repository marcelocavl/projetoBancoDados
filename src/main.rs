mod includes;

use includes::classes::Funcionario::Funcionario;
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

	let mut id_funcionarios:Vec<Funcionario>=Vec::new();

}
