
use std::fs;
use crate::includes::Funcionarios::Funcionarios;
use crate::includes::classes::Funcionario::Funcionario;

pub struct funcionarios_arquivotxt<'a>{
    funcionarios:Funcionarios, //lista de funcionarios
    path:             &'a str  //caminho para arquivo txt
}


impl <'a>funcionarios_arquivotxt<'a>{

    /******************************
			        FUNCAO NEW	
	  ******************************/

    //funcao construtora
    pub fn new(funcionarios:Funcionarios,path:&'a str)->Self{
        funcionarios_arquivotxt{
            funcionarios,
            path

        }
    }
    /******************************
			        FUNCOES GETS	
	  ******************************/
    //funcao de retornar classe funcionarios
    pub fn get_funcionarios(&mut self)->&mut Funcionarios{
        &mut self.funcionarios
    }

     //funcao de retornar path
    pub fn get_path(&mut self)->&str{
        self.path
    }

    //funcao de retornar lista de funcionarios (Vec<Funcionarios>)
    pub fn get_lista_funcionarios(&mut self)->&mut Vec<Funcionario>{
        self.get_funcionarios().get_lista()
    }

    pub fn get_tam_lista_funcionarios(&mut self)->&mut usize{
        self.get_funcionarios().get_num_funcionarios()
    }


                        

    /******************************
			        FUNCOES SETS	
	  ******************************/
    //funcao de retornar funcionarios
    pub fn set_funcionarios(&mut self,funcionarios:Funcionarios)->bool{

        self.funcionarios=funcionarios;
        true
    }
     //funcao de retornar path
    pub fn set_path(&mut self,path:&'a str)->bool{

        self.path=path;
        true

    }
    
    /******************************
		 FUNCOES ADICIONAR FUNCIONARIO
	  ******************************/  
    //funcao de adicionar funcionario ao vetor de funcionarios da classe Funcionarios
    pub fn adicionar_funcionario(&mut self,funcionario:Funcionario)->bool{
        self.get_funcionarios().adicionar_funcionario(funcionario);
        self.atualizar_txt();
        true
    }

    /******************************
			  FUNCOES ATUALIZAR TXT	
	  ******************************/   

    //funcao que ira atualizar o txt de acordo com as variaveis alocadas
    pub fn atualizar_txt(&mut self){
        let mut vector_funcionarios:Vec<Funcionario>=self.get_lista_funcionarios().clone();
        let mut num_funcionarios:usize=self.get_tam_lista_funcionarios().clone();
        let mut cont:usize=0;
        let mut atributos_funcionario:String=String::from("ID,NOME,CPF,ENDEREÇO,GENERO,NASCIMENTO,ID DEPARTAMENTO\n");
        fs::write(self.get_path(),"");
        //let mut atributos_funcionario:String=String::new();


        while cont<num_funcionarios {
            let prox_funcionario:&str=&vector_funcionarios[cont].colocar_atributos_em_string();
            atributos_funcionario.push_str(prox_funcionario);
            atributos_funcionario.push('\n');
            cont+=1;
        }
        let sla=self.get_path();
        println!("{sla}");
        match fs::write(self.get_path(), atributos_funcionario) {
        Ok(_) => println!("Arquivo salvo com sucesso."),
        Err(e) => eprintln!("Erro ao salvar arquivo: {}", e),
            
    }


}
}
