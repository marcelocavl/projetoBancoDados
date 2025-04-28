pub struct Departamento {
    pub id: u32,
    pub nome: String,
    pub id_gerente: u32,
}

impl Departamento {
    pub fn new(id: u32, nome: String, id_gerente: u32) -> Self {
        Departamento {
            id,
            nome,
            id_gerente,
        }
    }
}