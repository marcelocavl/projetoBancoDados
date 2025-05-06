// src/includes/utils/validator.rs
use super::{
    validator_funcionario::ValidatorFuncionario,
    validator_departamento::ValidatorDepartamento,
    validator_projeto::ValidatorProjeto
};

pub struct Validator {
    pub funcionario: ValidatorFuncionario,
    pub departamento: ValidatorDepartamento,
    pub projeto: ValidatorProjeto,
}

impl Validator {
    pub const fn new() -> Self {
        const FUNCIONARIO:ValidatorFuncionario = ValidatorFuncionario::new();
        const DEPARTAMENTO:ValidatorDepartamento= ValidatorDepartamento::new();
        const  PROJETO:ValidatorProjeto = ValidatorProjeto::new();
        Validator {
            funcionario:FUNCIONARIO,
            departamento:DEPARTAMENTO,
            projeto:PROJETO,
        }
    }
    pub fn string(&self,prompt: &str , validacao : fn(&str)-> bool , erro: &str,ler_input: fn(&str)->String) -> String {
        loop {
            let entrada: String = ler_input(prompt);
            if validacao(&entrada) {
                return entrada;
            } else {
                println!("{}", erro);
            }
        }
    }
    
    pub fn numero(&self,prompt: &str,  validacao : fn(u32)-> bool, erro: &str,ler_input: fn(&str)->String) -> u32 {
        loop {
            let entrada: u32= ler_input(prompt).parse().unwrap_or(0);
            if validacao(entrada) {
                return entrada;
            } else {
                println!("{}", erro);
            }
        }
    }
    
    pub fn float(&self,prompt: &str,  validacao : fn(f64)-> bool, erro: &str,ler_input: fn(&str)->String) -> f64 {
        loop {
            let entrada: f64= ler_input(prompt).parse().unwrap_or(0.0);
            if validacao(entrada) {
                return entrada;
            } else {
                println!("{}", erro);
            }
        }
    }
    
    pub fn char(&self,prompt: &str,  validacao : fn(char)-> bool, erro: &str,ler_input: fn(&str)->String) -> char {
        loop {
            let entrada: char= ler_input(prompt).chars().next().unwrap_or('?');
            if validacao(entrada) {
                return entrada;
            } else {
                println!("{}", erro);
            }
        }
    }
}