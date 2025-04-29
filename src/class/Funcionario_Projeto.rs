//relacao entre funcionario e projeto
pub struct FuncionarioProjeto {
    pub id_funcionario: u32, //id do funcionario que participa do projeto
    pub id_projeto: 		u32, //id do projeto no qual o funcionario trabalha
}

impl FuncionarioProjeto {
    //funcao construtora da struct FuncionarioProjeto
    pub fn new(id_funcionario: u32, id_projeto: u32) -> Self {
        Self {
            id_funcionario,
            id_projeto,
        }
    }

    /******************************
            FUNCOES GET
    ******************************/
    //funcao retorna o id do funcionario
    pub fn get_id_funcionario(&self) -> &u32 {
        &self.id_funcionario
    }

    //funcao retorna o id do projeto
    pub fn get_id_projeto(&self) -> &u32 {
        &self.id_projeto
    }

    /******************************
            FUNCOES SET
    ******************************/
    //funcao altera o id do funcionario
    pub fn set_id_funcionario(&mut self, id_funcionario: u32) -> bool {
        self.id_funcionario = id_funcionario;
        true
    }

    //funcao altera o id do projeto
    pub fn set_id_projeto(&mut self, id_projeto: u32) -> bool {
        self.id_projeto = id_projeto;
        true
    }

    /******************************
            FUNCAO PRINT
    ******************************/
    //funcao imprime os dados da relacao funcionario-projeto
    pub fn print(&self) {
        println!("ID FUNCIONARIO: {}", self.get_id_funcionario());
        println!("ID PROJETO: {}", self.get_id_projeto());
        println!("----------------------------");
    }
}
