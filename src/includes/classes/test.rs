mod Funcionario;

fn main(){
	let mut func=Funcionario::new(
												32,
												String::from("torres"),
												String::from("123"),
												String::from("rua ifce"),
												1200.0,
												'm',
												String::from("12/2/12")
												,23);

	func.print();
	func.set_id(456);
	func.print();

}
