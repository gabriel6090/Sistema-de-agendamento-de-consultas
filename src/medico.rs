use chrono::NaiveDate;
use std::io::{self, Write};

pub struct Medico {
    pub nome: String,
    pub data_nascimento: NaiveDate,
    pub senha: String,
    pub crm: String,
    pub especialidade: String,
    pub id: i32,
}

impl Medico {
    pub fn new(      
        nome: &str,
        data_nascimento: NaiveDate,
        senha: &str,
        crm: &str,
        especialidade: &str,
        id: i32,
    ) -> Self {
        Self {
            nome: nome.to_string(),
            data_nascimento,
            senha: senha.to_string(),
            crm: crm.to_string(),
            especialidade: especialidade.to_string(),
            id: id.clone(),
    }

    }
    pub fn get_nome(&self) -> &str{
        &self.nome
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_crm(&self) -> &str{
        &self.crm
    }

    pub fn get_data_nascimento(&self) -> String {
        self.data_nascimento.format("%d-%m-%Y").to_string()
    }

    pub fn get_especialidade(&self) -> &str {
        &self.especialidade
    }

    pub fn get_senha(&self) -> &str {
        &self.senha
    }

    pub fn login_medico(medicos: &Vec<Medico>) -> Option<&Medico> {
        let mut input = String::new();
    
        println!("Digite seu CRM:");
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        let crm = input.trim().to_string();
        input.clear();
    
        println!("Digite sua senha:");
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        let senha = input.trim().to_string();
    
        if let Some(medico) = medicos.iter().find(|m| m.crm == crm && m.get_senha() == senha) {
            println!(
                "Login bem-sucedido! Bem-vindo, Dr(a). {}, Especialidade: {}.",
                medico.get_nome(),
                medico.get_especialidade(),
            );
            Some(medico) // Retorna o médico logado
        } else {
            println!("Erro: CRM ou senha inválidos!");
            None // Retorna None se o login falhar
        }
    }
}