/*!!!!!!!!!!!!!!!!!!
	AINDA ESTOU IMPLEMENTANDO
	PROVAVELMENTE COM ALGUNS ERROS

!!!!!!!!!!!!!!!!!!!*/

use crate::includes::classes::Funcionario::Funcionario;
use crate::includes::classes::Projeto::Projeto;

//relacao entre funcionario e projeto
pub struct FuncionarioProjeto {
    pub ids_funcionarios:Vec<Funcionario>, //ids dos funcionarios que participam do projeto
    pub 				ids_projetos:Vec<Projeto>, //ids dos projetos nos quais os funcionarios trabalham
}

impl FuncionarioProjeto {
    //funcao construtora da struct FuncionarioProjeto
    pub fn new(ids_funcionarios:Vec<Funcionario>,ids_projetos:Vec<Projeto>) -> Self {
        Self {
            ids_funcionarios,
            ids_projetos,
        }
    }

    /******************************
            FUNCOES GET    
    ******************************/
    //funcao retorna o id do funcionario
    pub fn get_id_funcionario(&self) -> &Vec<Funcionario> {
        &self.ids_funcionarios
    }

    //funcao retorna o id do projeto
    pub fn get_id_projeto(&self) -> &Vec<Projeto> {
        &self.ids_projetos
    }

    /******************************
            FUNCOES SET    
    ******************************/
    //funcao altera o id do funcionario
    pub fn set_id_funcionario(&mut self,ids_funcionarios:Vec<Funcionario>) -> bool {
        self.ids_funcionarios = ids_funcionarios;
        true
    }

    //funcao altera o id do projeto
    pub fn set_id_projeto(&mut self,ids_projetos:Vec<Projeto>) -> bool {
        self.ids_projetos = ids_projetos;
        true
    }

    /******************************
            FUNCAO PRINT    
    ******************************/
/*
    //funcao imprime os dados da relacao funcionario-projeto
    pub fn print(&self) {
        println!("ID FUNCIONARIO: {}", self.get_id_funcionario());
        println!("ID PROJETO: {}", self.get_id_projeto());
        println!("----------------------------");
    }
*/
}

