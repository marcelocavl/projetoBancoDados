use crate::includes::classes::Funcionario::Funcionario;
use crate::includes::classes::Projeto::Projeto;

//relacao entre funcionario e projeto
pub struct FuncionarioProjeto {
    pub ids_funcionarios:Vec<Funcionario>, //ids dos funcionarios que participam do projeto
    pub 				ids_projetos:Vec<Projeto>, //ids dos projetos nos quais os funcionarios trabalham
		pub 							  num_relacoes:usize //numero de relacoes que existem na tabela
}

impl FuncionarioProjeto {
    //funcao construtora da struct FuncionarioProjeto
    pub fn new(ids_funcionarios:Vec<Funcionario>,ids_projetos:Vec<Projeto>,num_relacoes:usize) -> Self {
        Self {
            ids_funcionarios,
            ids_projetos,
						num_relacoes
        }
    }

    /******************************
            FUNCOES GET    
    ******************************/
    //funcao retorna o id do funcionario
    pub fn get_ids_funcionarios(&mut self) -> &mut Vec<Funcionario> {
        &mut self.ids_funcionarios
    }

    //funcao retorna o id do projeto
    pub fn get_ids_projetos(&mut self) -> &mut Vec<Projeto> {
        &mut self.ids_projetos
    }
		
		//funcao retorna o numero de relacoes
		pub fn get_num_relacoes(&mut self)-> &mut usize{
			&mut self.num_relacoes
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
     FUNCOES DE ADICIONAR RELACAO
    ******************************/
		//funcao para adicionar relacao funcionario-projeto na lista
		pub fn adicionar_relacao(&mut self,novo_funcionario:Funcionario,novo_projeto:Projeto)->bool{
			self.get_ids_funcionarios().push(novo_funcionario);	
			self.get_ids_projetos().push(novo_projeto);
			self.incrementar_num_relacoes();
			true
				
		}

		//funcao de incrementar o numero de relacoes
		pub fn incrementar_num_relacoes(&mut self){	
			self.num_relacoes+=1
		}

    /******************************
     	FUNCOES DE RETIRAR RELACAO
    ******************************/
/**!!!!!!!!!!!!!!!!!!!!!!!!!	
	COMENTEI A IMPLEMENTACAO DA FUNCAO DE DELETAR FUNCIONARIO POIS ESTÁ DANDO ALGUNS PROBLEMAS
	IREI AJEITAR MAIS PRO FINAL,PARA NÃO CONSUMIR MUITO TEMPO
!!!!!!!!!!!!!!!!!!!!!!!!!*/

		//funcao para remover relacao funcionario-projeto da lista
/*
		pub fn remover_relacao(&mut self,novo_funcionario:Funcionario,novo_projeto:Projeto)->bool{
			self.get_ids_funcionarios().retain(|x| x!= novo_funcionario);	
			self.get_ids_projetos().retain(|x| x!= novo_projeto);
			self.desincrementar_num_relacoes();
			true
				
		}

		//funcao de desincrementar o numero de relacoes
		pub fn desincrementar_num_relacoes(&mut self){	
			self.num_relacoes-=1
		}


*/
    /******************************
            FUNCAO PRINT    
    ******************************/
/*
	A FUNCAO ESTÁ FUNCIONANDO COM UM POUCO DE GAMBIARRA, NÃO IREI GASTAR MUITO TEMPO TENTANDO FAZER UMA SOLUÇÃO MAIS IDIOMATICA

	basicamente o rust não aceita multiplos borrows mutáveis dentro da função( não entendi muito bem,caso fosse eu atrás poderia perder mais tempo). Então utilizei o método clone para me livrar desse problema.
*/
	 pub fn print_ids_funcionarios_e_projetos(&mut self){
			 let mut ids_funcionarios=self.get_ids_funcionarios().clone();	//vetor de ids de funcionarios
			 let mut ids_projetos=self.get_ids_projetos().clone();					//vetor de ids de projetos
	 		 let mut num_relacoes=self.get_num_relacoes().clone(); 					//numero de relacoes da tabela funcionario-projeto
			 let mut cont:usize=0;													 								//contador do loop while
			
			 while cont<num_relacoes{
			  ids_funcionarios[cont].print_id();
				print!("||");
			  ids_projetos[cont].print_id_projeto();
				println!("");
				cont+=1;
			 }
 	 }	

}

