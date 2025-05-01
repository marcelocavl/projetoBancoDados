//funcionario da empresa

#[derive(Clone)]
pub  struct Funcionario {
     id: 							u32,	//id **UNICO** de cada funcionario
		 nome:				 String,
     cpf: 		 	 	 String,	//cpf **UNICO** do usuario
     endereco: 	 	 String,	//endereco do usuario
     salario: 				f64,	//salario do usuario
     genero: 		 	 	 char,	//genero do usuario
     nascimento: 	 String,	//data de nascimento do usuario
     id_departamento: u32,	//id do departamento no qual o funcionario participa
}

impl Funcionario {
		//funcao construtora da struct Funcionarios
    pub fn new(
        id: 						 u32,
        nome: 				String,
        cpf: 					String,
        endereco: 		String,
        salario: 				 f64,
        genero: 				char,
        nascimento: 	String,
        id_departamento: u32,
    ) -> Self {
        Funcionario {
            id,
            nome,
            cpf,
            endereco,
            salario,
            genero,
            nascimento,
            id_departamento,
        }
    }
	/******************************
			FUNCOES GET	
	******************************/
	//funcao retorna id do funcionario
	pub fn get_id(&self)->&u32{
		&self.id
	}

	//funcao retorna nome do funcionario
	pub fn get_nome(&self)->&String{
		&self.nome
	}

	//funcao retorna cpf do funcionario
	pub fn get_cpf(&self)->&String{
		&self.cpf
	}

	//funcao retorna id do funcionario
	pub fn get_endereco(&self)->&String{
		&self.endereco
	}

	//funcao retorna id do funcionario
	pub fn get_salario(&self)->&f64{
		&self.salario
	}

	//funcao retorna id do funcionario
	pub fn get_genero(&self)->&char{
		&self.genero
	}

	//funcao retorna id do funcionario
	pub fn get_nascimento(&self)->&String{
		&self.nascimento
	}

	//funcao retorna id do funcionario
	pub fn get_id_departamento(&self)->&u32{
		&self.id_departamento
	}

	/******************************
			FUNCOES SET	
	******************************/

	//funcao retorna id do funcionario
	pub fn set_id(&mut self,id:u32)->bool{
		self.id=id;		
		true
	}

	//funcao retorna nome do funcionario
	pub fn set_nome(&mut self,nome:String)->bool{
		self.nome=nome;
		true	
	}

	//funcao retorna cpf do funcionario
	pub fn set_cpf(&mut self,cpf:String)->bool{
		self.cpf=cpf;
		true
	}

	//funcao retorna id do funcionario
	pub fn set_endereco(&mut self,endereco:String)->bool{
		self.endereco=endereco;		
		true
	}

	//funcao retorna id do funcionario
	pub fn set_salario(&mut self,salario:f64)->bool{	
		self.salario=salario;
		true
	}

	//funcao retorna id do funcionario
	pub fn set_genero(&mut self,genero:char)->bool{
		self.genero=genero;
		true
	}

	//funcao retorna id do funcionario
	pub fn set_nascimento(&mut self,nascimento:String)->bool{
		self.nascimento=nascimento;
		true
	}

	//funcao retorna id do funcionario
	pub fn set_id_departamento(&mut self,id_departamento:u32)->bool{
		self.id_departamento=id_departamento;
		true
	}

	/******************************
			FUNCOES PRINT	
	******************************/
//funcao para printar o usuario
	pub fn print(&self){
		self.print_id();
		println!("");
		self.print_nome();
		println!("");
		self.print_cpf();
		println!("");
		self.print_endereco();
		println!("");
		self.print_genero();	
		println!("");
		self.print_salario();
		println!("");
		self.print_nascimento();
		println!("");
		self.print_id_departamento();
		println!("");
		println!("----------------------------");
	}

//funcao para printar id funcionario
	pub fn print_id(&self){
					print!("ID FUNCIONARIO:{}",self.get_id());
	}

//funcao para printar nome funcionario
	pub fn print_nome(&self){
					print!("NOME:{}",self.get_nome());
	}

//funcao para printar cpf usuario	
	pub fn print_cpf(&self){
					print!("CPF:{}",self.get_cpf());
	}

//funcao para printar endereco usuario
	pub fn print_endereco(&self){
					print!("ENDERECO:{}",self.get_endereco());
	}

//funcao para printar genero usuario
	pub fn print_genero(&self){
					print!("GENERO:{}",self.get_genero());
	}

//funcao para printar salario usuario
	pub fn print_salario(&self){
					print!("SALARIO:{}",self.get_salario());
	}

//funcao para printar nascimento usuario
	pub fn print_nascimento(&self){
					print!("NASCIMENTO:{}",self.get_nascimento());
	}

//funcao para printar id do departamento do usuario
	pub fn print_id_departamento(&self){
					print!("ID DEPARTAMENTO:{}",self.get_id_departamento());
	}


}

