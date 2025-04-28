pub struct FuncionarioProjeto {
    pub id_funcionario: u32,
    pub id_projeto: u32,
}

impl FuncionarioProjeto {
    pub fn new(id_funcionario: u32, id_projeto: u32) -> Self {
        Self {
            id_funcionario,
            id_projeto,
        }
    }
}