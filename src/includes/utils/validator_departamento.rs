pub struct ValidatorDepartamento;

impl ValidatorDepartamento {
    pub const fn new() -> Self {
        ValidatorDepartamento {}
    }

    pub fn nome(&self,nome: &str) -> bool {
        if nome.len() < 3 {
            return false;
        }
        return true;
    }
    pub fn id_gerente(&self,id_gerente: u32) -> bool {
        return id_gerente > 0
    }

}