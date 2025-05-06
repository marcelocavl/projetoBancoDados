// Validator.rs

pub struct Validator_Funcionario;

impl Validator_Funcionario {
    pub const fn new() -> Self {
        Validator_Funcionario {}
    }
    pub fn cpf(&self,cpf: &str) -> bool {
        let cpf = cpf.chars().filter(|c| c.is_digit(10)).collect::<String>();

        if cpf.len() != 11 || cpf.chars().all(|c| c == cpf.chars().next().unwrap()) {
            return false;
        }

        let digits: Vec<u32> = cpf.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        // Função para calcular um dígito verificador
        let calc_digit = |slice: &[u32], factor_start: u32| -> u32 {
            let sum = slice.iter()
                .enumerate()
                .map(|(i, &num)| num * (factor_start - i as u32))
                .sum::<u32>();
            
            let remainder = sum % 11;
            if remainder < 2 { 0 } else { 11 - remainder }
        };

        // Calcula o primeiro dígito verificador (10 → 2)
        let first_digit = calc_digit(&digits[0..9], 10);
        
        // Calcula o segundo dígito verificador (11 → 2)
        let second_digit = calc_digit(&digits[0..10], 11);

        // Verifica se os dígitos calculados batem com os informados
        return first_digit == digits[9] && second_digit == digits[10];
    }
    pub fn gender(gender: char) -> bool {
        if(gender != 'M' || gender != 'F'){
            return false;
        }
        return true;
    }
    pub fn salario(salario: f64) -> bool {
        if(salario < 0.0){
            return false;
        }
        return true;
    }
    pub fn nascimento(nascimento: &str) -> bool {
        let parts: Vec<&str> = nascimento.split('/').collect();
        if parts.len() != 3 {
            return false;
        }
        let dia: u32 = parts[0].parse().unwrap_or(0);
        let mes: u32 = parts[1].parse().unwrap_or(0);
        let ano: u32 = parts[2].parse().unwrap_or(0);

        if dia < 1 || dia > 31 || mes < 1 || mes > 12 || ano < 1900 {
            return false;
        }
        return true;
    }
    pub fn endereco(endereco: &str) -> bool {
        if endereco.len() < 5 {
            return false;
        }
        return true;
    }
    pub fn nome(nome: &str) -> bool {
        if nome.len() < 3 {
            return false;
        }
        return true;
    }
    pub fn id_departamento(id_departamento: u32) -> bool {
        if id_departamento < 1 {
            return false;
        }
        return true;
    }

}
