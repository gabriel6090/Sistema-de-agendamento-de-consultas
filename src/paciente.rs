use chrono::NaiveDate;
use std::{clone, io::{self, Write}};
use std::sync::atomic::{AtomicI32, Ordering};

pub struct Paciente {
    pub cpf: String,
    pub nome: String,
    pub data_nascimento: NaiveDate,
    pub senha: String, 
    pub id: i32,
}

static ID_COUNTER: AtomicI32 = AtomicI32::new(1);

impl Paciente {
    pub fn new(cpf: &str, nome: &str, data_nascimento: NaiveDate, senha: &str, id: i32) -> Self {
        Self {
            cpf: cpf.to_string(),
            nome: nome.to_string(),
            data_nascimento,
            senha: senha.to_string(),
            id: id.clone(),
        }
    }

    pub fn get_cpf(&self) -> &str {
        &self.cpf
    }
    
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_nome(&self) -> &str {
        &self.nome
    }

    pub fn get_data_nascimento(&self) -> NaiveDate {
        self.data_nascimento
    }

    pub fn get_senha(&self) -> &str {
        &self.senha
    }

    pub fn registro() -> Result<Self, chrono::ParseError> {
        let mut input = String::new();

    let cpf = loop {
        println!("Digite seu CPF (apenas números, 11 dígitos):");
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        let cpf = input.trim().to_string();
        input.clear();

        if cpf.len() == 11 && cpf.chars().all(|c| c.is_digit(10)) {
            break cpf;
        } else {
            println!("CPF inválido! Certifique-se de que possui exatamente 11 dígitos numéricos.");
        }
    };

    println!("Digite seu nome:");
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    let nome = input.trim().to_string();
    input.clear();

    println!("Digite sua data de nascimento (formato DD-MM-YYYY):");
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    let data_nascimento = NaiveDate::parse_from_str(input.trim(), "%d-%m-%Y")?;
    input.clear();

    println!("Digite sua senha:");
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    let senha = input.trim().to_string();

    let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    

    Ok(Self::new(&cpf, &nome, data_nascimento, &senha, id))
    }
}