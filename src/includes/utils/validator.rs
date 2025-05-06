// src/includes/utils/validator.rs
use super::{
    validator_funcionario::Validator_Funcionario,
    validator_departamento::Validator_Departamento,
    validator_projeto::Validator_Projeto
};

pub struct Validator {
    pub funcionario: Validator_Funcionario,
    pub departamento: Validator_Departamento,
    pub projeto: Validator_Projeto,
}

impl Validator {
    pub const fn new() -> Self {
        const FUNCIONARIO:Validator_Funcionario = Validator_Funcionario::new();
        const DEPARTAMENTO:Validator_Departamento= Validator_Departamento::new();
        const  PROJETO:Validator_Projeto = Validator_Projeto::new();
        Validator {
            funcionario:FUNCIONARIO,
            departamento:DEPARTAMENTO,
            projeto:PROJETO,
        }
    }
    
}