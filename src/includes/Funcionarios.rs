use crate::includes::classes::Funcionario::Funcionario;

// Lista de objetos do tipo Departamento
#[derive(Clone)]
pub struct Funcionarios {
    pub lista: Vec<Funcionario>, // Lista de departamentos
		pub num_funcionarios:usize		// Numero de funcionarios da lista
}

impl Funcionarios {
    /******************************
            FUNCAO NEW
    ******************************/
    // Construtor da struct Funcionarios
    pub fn new(lista: Vec<Funcionario>,num_funcionarios:usize) -> Self {
        Self { lista,
							 num_funcionarios }
    }

    /******************************
            FUNCOES GET
    ******************************/
    // Retorna a lista de funcionarios
    pub fn get_lista(&mut self) -> &mut Vec<Funcionario> {
        &mut self.lista
    }
	
		pub fn get_num_funcionarios(&mut self)->&mut usize{
			&mut self.num_funcionarios
		}
    /******************************
            FUNCOES SET
    ******************************/

    // Altera a lista de funcionarios
    pub fn set_lista(&mut self, lista: Vec<Funcionario>) -> bool {
        self.lista = lista;
        true
    }

    /******************************
     FUNCOES ADICIONAR FUNCIONARIO
    ******************************/
		//funcao para adicionar funcionario na lista
		pub fn adicionar_funcionario(&mut self,funcionario:Funcionario)->bool{
			self.get_lista().push(funcionario);	
			self.incrementa_num_funcionario();
			true
		}

		//funcao para incrementar numero de funcionarios
		pub fn incrementa_num_funcionario(&mut self){		
			self.num_funcionarios+=1;
		}

    /******************************
     FUNCOES DELETAR FUNCIONARIO
    ******************************/	
/**!!!!!!!!!!!!!!!!!!!!!!!!!	
	FIX ME
	COMENTEI A IMPLEMENTACAO DA FUNCAO DE DELETAR FUNCIONARIO POIS ESTÁ DANDO ALGUNS PROBLEMAS
	IREI AJEITAR MAIS PRO FINAL,PARA NÃO CONSUMIR MUITO TEMPO
!!!!!!!!!!!!!!!!!!!!!!!!!*/

/*
		//funcao para deletar um funcionario da lista
		pub fn achar_funcionario(&mut self,funcionario:&mut Funcionario)->&mut Funcionario{
			let mut lista_funcionarios=self.get_lista().clone();
			let mut num_funcionarios=self.get_num_funcionarios().clone();
			let mut cont:usize=0;
			while cont<num_funcionarios{
				if funcionario.cmp_funcionarios(&lista_funcionarios[cont]){
					return lista_funcionarios[cont];
				}
				cont+=1;
			}	

		}

			
		//funcao para desincrementar numero de funcionarios	
		pub fn desincrementa_num_funcionario(&mut self){		
			self.num_funcionarios-=1;
		}
*/
    /******************************
            FUNCAO PRINT
    ******************************/
    // Imprime os dados de todos os funcionarios na lista
    pub fn print_lista(&mut self) {
        for item in &mut self.lista {
            item.print();
        }
    }
}

