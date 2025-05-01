
use crate::includes::classes::Departamento::Departamento;

// Lista de objetos do tipo Departamento
pub struct Departamentos {
    pub lista: Vec<Departamento>, // Lista de departamentos
    pub num_departamentos: usize, // Numero de departamentos
}

impl Departamentos {
    /******************************
            FUNCAO NEW
    ******************************/
    // Construtor da struct Departamentos
    pub fn new(lista: Vec<Departamento>, num_departamentos: usize) -> Self {
        Self { lista,
							 num_departamentos }
    }

    /******************************
            FUNCOES GET
    ******************************/
    // Retorna a lista de departamentos
    pub fn get_lista(&self) -> &Vec<Departamento> {
        &self.lista
    }

    // Retorna o numero de departamentos na lista
    pub fn get_num_departamentos(&self) -> usize {
        self.num_departamentos
    }

    /******************************
            FUNCOES SET
    ******************************/
    // Altera a lista de departamentos
    pub fn set_lista(&mut self, lista: Vec<Departamento>) -> bool {
        self.lista = lista;
        true
    }

    // Altera o numero de departamentos
    pub fn set_num_departamentos(&mut self, num_departamentos: usize) -> bool {
        self.num_departamentos = num_departamentos;
        true
    }

    /******************************
     FUNCAO ADICIONAR DEPARTAMENTO
    ******************************/
    // Funcao de adicionar departamento
    pub fn adicionar_departamento(&mut self, departamento: Departamento) -> bool {
        self.lista.push(departamento);
        true
    }

    // Incrementa o numero de departamentos
    pub fn incrementa_num_departamentos(&mut self) {
        self.num_departamentos += 1;
    }

    /******************************
            FUNCAO PRINT
    ******************************/
    // Imprime os dados de todos os departamentos na lista
    pub fn print_lista(&self) {
        for item in &self.lista {
            item.print();
        }
    }
}

