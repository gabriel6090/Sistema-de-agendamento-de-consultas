use chrono::NaiveDate;
use std::io;
pub struct Paciente{
    pub cpf: u64,
    pub nome: String,
    pub data_nascimento: NaiveDate

}

impl Paciente {
    pub fn new(){
        let mut nome = String::new();
        let mut data_nascimento=String::new();
        let mut cpf =String::new();

        println!("Digite seu nome:");
        io::stdin().read_line(&mut nome).expect("Erro ao ler entrada");
        let nome = nome.trim();

        println!("Digite seu cpf:");
        io::stdin().read_line(&mut cpf).expect("Erro ao ler entrada");
        let cpf = cpf.trim();

        println!("Digite sua data de nascimento (AAAA-MM--DD):");
        io::stdin().read_line(&mut data_nascimento).expect("Erro ao ler entrada");
        let data_nascimento = data_nascimento.trim();
        match NaiveDate::parse_from_str(data_nascimento, "%d-%m-%Y"){
            Ok(data) => println!("Data válida: {}", data),
            Err(e) => println!("Formato inválido! Use AAAA-MM-DD\n erro: {}", e),
        }

        println!("Seu registro então é:\nNome: {}\nCpf: {}\nData de nascimento: {}", nome, cpf, data_nascimento);
     }

}

fn paciente(){

}

pub fn ler_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");

    input.trim().parse().unwrap_or_else(|_| {
        println!("Entrada inválida ! Digite um número.");
        ler_int()
    })
}