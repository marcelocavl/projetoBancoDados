// Validator.rs

pub struct ValidatorFuncionario;

impl ValidatorFuncionario {
    pub const fn new() -> Self {
        ValidatorFuncionario {}
    }
    pub fn cpf(&self,cpf: &str) -> bool {
        if cpf.len() != 14 {
            return false;
        }
        if !cpf.chars().all(|c| c.is_digit(10) || c == '.' || c == '-') {
            return false;
        }
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
    pub fn gender(&self,gender: char) -> bool {
        if gender != 'M' && gender != 'F'{
            return false;
        }
        return true;
    }
    pub fn salario(&self,salario: f64) -> bool {
        if salario < 0.0 {
            return false;
        }
        
        return true;
    }
    pub fn nascimento(&self,nascimento: &str) -> bool {
        let parts: Vec<&str> = nascimento.split('-').collect();
        if parts.len() != 3 {
            return false;
        }

        if parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
            return false;
        }
        if !parts[0].chars().all(|c| c.is_digit(10)) ||
           !parts[1].chars().all(|c| c.is_digit(10)) ||
           !parts[2].chars().all(|c| c.is_digit(10)) {
            return false;
        }
        let ano: u32 = parts[0].parse().unwrap_or(0);
        let mes: u32 = parts[1].parse().unwrap_or(0);
        let dia: u32 = parts[2].parse().unwrap_or(0);

        if dia < 1 || dia > 31 || mes < 1 || mes > 12 || ano < 1900 {
            return false;
        }
        return true;
    }
    pub fn endereco(&self,endereco: &str) -> bool {
        if endereco.len() < 5 {
            return false;
        }
        return true;
    }
    pub fn nome(&self,nome: &str) -> bool {
        if nome.len() < 3 {
            return false;
        }
        return true;
    }
    pub fn id_departamento(&self,id_departamento: u32) -> bool {
        if id_departamento < 1 {
            return false;
        }
        return true;
    }

}
