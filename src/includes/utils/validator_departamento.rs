pub struct Validator_Departamento;

impl Validator_Departamento {
    pub const fn new() -> Self {
        Validator_Departamento {}
    }

    pub fn validate_id(id: u32) -> bool {
        return id > 0
    }
    pub fn nome(nome: &str) -> bool {
        if nome.len() < 3 {
            return false;
        }
        return true;
    }
    pub fn validate_id_gerente(id_gerente: u32) -> bool {
        return id_gerente > 0
    }

}