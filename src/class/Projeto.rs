pub struct Projeto {
    pub id_projeto: u32,
    pub nome_projeto: String,
    pub id_departamento: u32,
    pub local: String,
}

impl Projeto {
    pub fn new(id_projeto: u32, nome_projeto: String, id_departamento: u32, local: String) -> Self {
        Self {
            id_projeto,
            nome_projeto,
            id_departamento,
            local,
        }
    }
}