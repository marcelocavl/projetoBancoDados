pub struct ValidatorProjeto;

impl ValidatorProjeto {
    pub const fn new() -> Self {
        ValidatorProjeto{}
    }
    pub fn validate_id(&self,id: u32) -> bool {
        return id > 0
    }

    pub fn nome(&self,nome: &str) -> bool {
        if nome.len() < 3 {
            return false;
        }
        return true;
    }

    pub fn id_departamento(&self,id_departamento: u32) -> bool {
        return id_departamento > 0
    }

    pub fn local(&self,local: &str) -> bool {
        if local.len() < 5 {
            return false;
        }
        return true;
    }
}