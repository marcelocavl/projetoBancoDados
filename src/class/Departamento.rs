//departamento da empresa
pub struct Departamento {
    pub id:					u32,     //id **UNICO** de cada departamento
    pub nome: 	 String,     //nome do departamento
    pub id_gerente: u32,     //id do gerente responsável pelo departamento
}

impl Departamento {
    //funcao construtora da struct Departamento
    pub fn new(id: u32, nome: String, id_gerente: u32) -> Self {
        Departamento {
            id,
            nome,
            id_gerente,
        }
    }

    /******************************
            FUNCOES GET
    ******************************/
    //funcao retorna id do departamento
    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    //funcao retorna nome do departamento
    pub fn get_nome(&self) -> &String {
        &self.nome
    }

    //funcao retorna id do gerente
    pub fn get_id_gerente(&self) -> &u32 {
        &self.id_gerente
    }

    /******************************
            FUNCOES SET
    ******************************/
    //funcao altera o id do departamento
    pub fn set_id(&mut self, id: u32) -> bool {
        self.id = id;
        true
    }

    //funcao altera o nome do departamento
    pub fn set_nome(&mut self, nome: String) -> bool {
        self.nome = nome;
        true
    }

    //funcao altera o id do gerente
    pub fn set_id_gerente(&mut self, id_gerente: u32) -> bool {
        self.id_gerente = id_gerente;
        true
    }

    /******************************
            FUNCAO PRINT
    ******************************/
    //funcao imprime os dados do departamento
    pub fn print(&self) {
        println!("ID: {}", self.get_id());
        println!("NOME: {}", self.get_nome());
        println!("ID GERENTE: {}", self.get_id_gerente());
        println
