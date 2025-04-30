//funcionario da empresa


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

	pub fn print(&self){
		println!("ID:{}",self.get_id());
		println!("NOME:{}",self.get_nome());
		println!("CPF:{}",self.get_cpf());
		println!("ENDERECO:{}",self.get_endereco());
		println!("GENERO:{}",self.get_genero());
		println!("SALARIO:{}",self.get_salario());
		println!("NASCIMENTO:{}",self.get_nascimento());
		println!("ID DEPARTAMENTO:{}",self.get_id_departamento());
		println!("----------------------------");
	}
				
}

