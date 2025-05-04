mod includes;
mod menu;

use includes::Funcionario_Projeto::FuncionarioProjeto;
use includes::Funcionarios::Funcionarios;
use includes::classes::Funcionario::Funcionario;
use includes::classes::Projeto::Projeto;
use includes::editores_txt::funcionarios_arquivotxt::funcionarios_arquivotxt;
use menu::*;

fn main() {
    let vec_func: Vec<Funcionario> = Vec::new();
    let lista_func: Funcionarios = Funcionarios::new(vec_func, 0);
    let path: &str = "./src/arquivos_txt/funcionarios.txt";

    let mut func_arquivotxt = funcionarios_arquivotxt::new(lista_func, path);

    while true {
        criar_funcionario(&mut func_arquivotxt);
    }
}
