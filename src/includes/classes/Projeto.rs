//projeto da empresa

#[derive(Clone)]
pub struct Projeto {
    pub id_projeto: u32,         //id **UNICO** do projeto
    pub nome_projeto: String,    //nome do projeto
    pub id_departamento: u32,    //id do departamento responsável pelo projeto
    pub local: String,           //local onde o projeto está sendo desenvolvido
}

impl Projeto {
    //funcao construtora da struct Projeto
    pub fn new(id_projeto: u32, nome_projeto: String, id_departamento: u32, local: String) -> Self {
        Self {
            id_projeto,
            nome_projeto,
            id_departamento,
            local,
        }
    }

    /******************************
            FUNCOES GET    
    ******************************/
    //funcao retorna id do projeto
    pub fn get_id_projeto(&self) -> &u32 {
        &self.id_projeto
    }

    //funcao retorna nome do projeto
    pub fn get_nome_projeto(&self) -> &String {
        &self.nome_projeto
    }

    //funcao retorna id do departamento associado
    pub fn get_id_departamento(&self) -> &u32 {
        &self.id_departamento
    }

    //funcao retorna local do projeto
    pub fn get_local(&self) -> &String {
        &self.local
    }

    /******************************
            FUNCOES SET    
    ******************************/
    //funcao altera id do projeto
    pub fn set_id_projeto(&mut self, id_projeto: u32) -> bool {
        self.id_projeto = id_projeto;
        true
    }

    //funcao altera nome do projeto
    pub fn set_nome_projeto(&mut self, nome_projeto: String) -> bool {
        self.nome_projeto = nome_projeto;
        true
    }

    //funcao altera id do departamento associado
    pub fn set_id_departamento(&mut self, id_departamento: u32) -> bool {
        self.id_departamento = id_departamento;
        true
    }

    //funcao altera local do projeto
    pub fn set_local(&mut self, local: String) -> bool {
        self.local = local;
        true
    }

    /******************************
            FUNCAO PRINT    
    ******************************/
    //funcao imprime os dados do projeto
    pub fn print(&self) {
				self.print_id_projeto();
				println!("");
				self.print_nome_projeto();
				println!("");
				self.print_id_departamento();
				println!("");
				self.print_local();
				println!("");
        println!("----------------------------");
    }
	//funcao para printar id do projeto
		pub fn print_id_projeto(&self){	
        print!("ID PROJETO: {}", self.get_id_projeto());
		}
	//funcao para printar nome do projeto	
		pub fn print_nome_projeto(&self){
        print!("ID DEPARTAMENTO: {}", self.get_id_departamento());
		}
	//funcao para printar id do departamento
		pub fn print_id_departamento(&self){
        print!("ID DEPARTAMENTO: {}", self.get_id_departamento());
		}
	//funcao para printar local
		pub fn print_local(&self){
        print!("LOCAL: {}", self.get_local());
    }
}

