pub struct Funcionario {
    pub id: u32,
    pub cpf: String,
    pub endereco: String,
    pub salario: f64,
    pub genero: char,
    pub nascimento: String,
    pub id_departamento: u32,
}

impl Funcionario {
    pub fn new(
        id: u32,
        nome: String,
        cpf: String,
        endereco: String,
        salario: f64,
        genero: char,
        nascimento: String,
        id_departamento: u32,
    ) -> Self {
        Funcionario {
            id,
            nome,
            cpf,
            endereco,
            salario,
            genero,
            nascimento,
            id_departamento,
        }
    }
}