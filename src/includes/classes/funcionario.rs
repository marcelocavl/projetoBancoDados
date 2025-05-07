// Funcionário da Empresa

#[derive(Clone)]
pub struct Funcionario {
    id: u32, //id **UNICO** de cada funcionario
    nome: String,
    cpf: String, //cpf **UNICO** do usuario
    endereco: String, //endereco do usuario
    salario: f64, //salario do usuario
    genero: char, //genero do usuario
    nascimento: String, //data de nascimento do usuario
    id_departamento: u32, //id do departamento no qual o funcionario participa
}

impl Funcionario {
    // Função construtora da struct Funcionario
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

    /******************************
            FUNCOES GET
    ******************************/

    pub fn get_id(&mut self) -> &mut u32 {
        &mut self.id
    }
    pub fn get_nome(&mut self) -> &mut String {
        &mut self.nome
    }
    pub fn get_cpf(&mut self) -> &mut String {
        &mut self.cpf
    }
    pub fn get_endereco(&mut self) -> &mut String {
        &mut self.endereco
    }
    pub fn get_salario(&mut self) -> &mut f64 {
        &mut self.salario
    }
    pub fn get_genero(&mut self) -> &mut char {
        &mut self.genero
    }
    pub fn get_nascimento(&mut self) -> &mut String {
        &mut self.nascimento
    }
    pub fn get_id_departamento(&mut self) -> &mut u32 {
        &mut self.id_departamento
    }

    /******************************
            FUNCOES SET
    ******************************/

    // pub fn set_id(&mut self, id: u32) -> bool {
    //     self.id = id;
    //     true
    // }
    pub fn set_nome(&mut self, nome: String) -> bool {
        self.nome = nome;
        true
    }
    pub fn set_cpf(&mut self, cpf: String) -> bool {
        self.cpf = cpf;
        true
    }
    pub fn set_endereco(&mut self, endereco: String) -> bool {
        self.endereco = endereco;
        true
    }
    pub fn set_salario(&mut self, salario: f64) -> bool {
        self.salario = salario;
        true
    }
    pub fn set_genero(&mut self, genero: char) -> bool {
        self.genero = genero;
        true
    }
    pub fn set_nascimento(&mut self, nascimento: String) -> bool {
        self.nascimento = nascimento;
        true
    }
    pub fn set_id_departamento(&mut self, id_departamento: u32) -> bool {
        self.id_departamento = id_departamento;
        true
    }

    /******************************
            FUNCOES DE COMPARACAO
    ******************************/

    // pub fn cmp_funcionarios(&mut self, funcionario: &mut Funcionario) -> bool {
    //     if self.get_id() != funcionario.get_id() {
    //         return false;
    //     }
    //     if self.get_nome() != funcionario.get_nome() {
    //         return false;
    //     }
    //     if self.get_cpf() != funcionario.get_cpf() {
    //         return false;
    //     }
    //     if self.get_endereco() != funcionario.get_endereco() {
    //         return false;
    //     }
    //     if self.get_salario() != funcionario.get_salario() {
    //         return false;
    //     }
    //     if self.get_genero() != funcionario.get_genero() {
    //         return false;
    //     }
    //     if self.get_nascimento() != funcionario.get_nascimento() {
    //         return false;
    //     }
    //     if self.get_id_departamento() != funcionario.get_id_departamento() {
    //         return false;
    //     }
    //     true
    // }

    /******************************
            FUNCOES PRINT
    ******************************/

    // pub fn print(&mut self) {
    //     self.print_id();
    //     println!("");
    //     self.print_nome();
    //     println!("");
    //     self.print_cpf();
    //     println!("");
    //     self.print_endereco();
    //     println!("");
    //     self.print_genero();
    //     println!("");
    //     self.print_salario();
    //     println!("");
    //     self.print_nascimento();
    //     println!("");
    //     self.print_id_departamento();
    //     println!("");
    //     println!("----------------------------");
    // }

    // pub fn print_id(&mut self) {
    //     print!("ID FUNCIONARIO:{}", self.get_id());
    // }
    // pub fn print_nome(&mut self) {
    //     print!("NOME:{}", self.get_nome());
    // }
    // pub fn print_cpf(&mut self) {
    //     print!("CPF:{}", self.get_cpf());
    // }
    // pub fn print_endereco(&mut self) {
    //     print!("ENDERECO:{}", self.get_endereco());
    // }
    // pub fn print_genero(&mut self) {
    //     print!("GENERO:{}", self.get_genero());
    // }
    // pub fn print_salario(&mut self) {
    //     print!("SALARIO:{}", self.get_salario());
    // }
    // pub fn print_nascimento(&mut self) {
    //     print!("NASCIMENTO:{}", self.get_nascimento());
    // }
    // pub fn print_id_departamento(&mut self) {
    //     print!("ID DEPARTAMENTO:{}", self.get_id_departamento());
    // }

    // funcao de colocar atributos do funcionario em uma string unica
    pub fn colocar_atributos_em_string(&mut self) -> String {
        let mut atributos: String = String::new(); //instanciei uma string vazia

        let string_id: String = self.get_id().to_string().clone(); //transformando id:u32 em id:String
        let str_id: &str = &string_id; //transformando id:String em id:&str

        let string_id_departamento: String = self.get_id_departamento().to_string().clone(); //transformando id_departamento:u32 em String
        let str_id_departamento: &str = &string_id_departamento; //transformando id_departamento:String em &str

        let nome: &str = &self.get_nome().clone();
        let cpf: &str = &self.get_cpf().clone();
        let endereco: &str = &self.get_endereco().clone();
        let gender: char = self.get_genero().clone();
        let nascimento: &str = &self.get_nascimento().clone();

        atributos.push_str(str_id);
        atributos.push(',');
        atributos.push_str(nome);
        atributos.push(',');
        atributos.push_str(cpf);
        atributos.push(',');
        atributos.push_str(endereco);
        atributos.push(',');
        atributos.push(gender);
        atributos.push(',');
        atributos.push_str(nascimento);
        atributos.push(',');
        atributos.push_str(str_id_departamento);

        atributos
    }
}
