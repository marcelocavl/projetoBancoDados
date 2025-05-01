use crate::includes::classes::Projeto::Projeto;

// Lista de objetos do tipo Projeto
pub struct Projetos {
    pub lista: Vec<Projeto>, // Lista de projetos
		pub  num_projetos:usize, // Numero de projetos
}

impl Projetos {
    /******************************
            FUNCAO NEW
    ******************************/
    // Construtor da struct Projetos
    pub fn new(lista: Vec<Projeto>,num_projetos:usize) -> Self {
        Self { lista, 
						   num_projetos}
    }

    /******************************
            FUNCOES GET
    ******************************/
    // Retorna a lista de projetos
    pub fn get_lista(&self) -> &Vec<Projeto> {
        &self.lista
    }
	
		//Retorna o numero de projetos na lista
		pub fn get_num_projetos(&self)->&Vec<projeto>{	
				&self.num_projetos
		}	
    /******************************
            FUNCOES SET
    ******************************/
    // Altera a lista de projetos
    pub fn set_lista(&mut self, lista: Vec<Projeto>) -> bool {
        self.lista = lista;
        true
    }
		// Altera o numero de projetos
    pub fn set_num_projetos(&mut self,num_projetos:usize) -> bool {
        self.num_projetos =num_projetos;
        true
    }

    /******************************
      FUNCAO ADICIONAR PROJETO
    ******************************/
		//funcao de adicionar projeto
		pub fn adicionar_projeto(&mut self,projeto:Projeto)->bool{
			self.get_lista().push(projeto);	
			true
		}

		pub fn incrementa_num_projetos(&mut self){		
			self.num_projetos+=1;
		}

    /******************************
            FUNCAO PRINT
    ******************************/
    // Imprime os dados de todos os projetos na lista
    pub fn print_lista(&self) {
        for item in &self.lista {
            item.print();
        }
    }
}

