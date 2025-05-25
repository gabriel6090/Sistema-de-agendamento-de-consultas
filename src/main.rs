use chrono::NaiveDate;
use std::io;
mod paciente;
mod medico;
mod consulta;
mod database;
use crate::medico::Medico;
use crate::database::Database;

//função para ler inputs do tipo int do usuario
fn ler_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");

    input.trim().parse().unwrap_or_else(|_| {
        println!("Entrada inválida ! Digite um número.");
        ler_int()
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection
    let database = Database::new("mysql://root:Gabriel2010!@localhost/agendamento?socket=/var/run/mysqld/mysqld.sock").await?;
    
    // Create tables if they don't exist
    database.criar_tabelas().await?;

    let mut login: bool = false;
    let mut medico_login = false;
    let mut consultas: Vec<consulta::Consulta> = Vec::new();
    let mut paciente_logado_id: Option<i32> = None;
    let mut medico_logado_id: Option<i32> = None;
    
    //paciente teste
    let mut pacientes: Vec<paciente::Paciente> = vec![
    paciente::Paciente::new(
        "12345678900",
        "João Silva",
        NaiveDate::from_ymd(1990, 1, 1),
        "senha123",
        1,
    ),
    ];

    //Medicos teste
    let mut medicos: Vec<Medico> = vec![
    Medico::new(
        "Dr. João Silva",
        NaiveDate::from_ymd(1980, 5, 20),
        "senha123",
        "CRM12345",
        "Cardiologia",
        1,
    ),
    Medico::new(
        "Dra. Maria Oliveira",
        NaiveDate::from_ymd(1975, 8, 15),
        "senha456",
        "CRM67890",
        "Pediatria",
        2,
    ),
    Medico::new(
        "Dr. Carlos Souza",
        NaiveDate::from_ymd(1990, 3, 10),
        "senha789",
        "CRM54321",
        "Ortopedia",
        3,
    ),
    ];

    //Interface de login de paciente
    let mut op: i32;
            println!("Bem-vindo ao sistema de agendamento de consultas!");
    loop {
        println!("Logar como:\n1 - Paciente\n2 - Médico\n3 - Sair");
        op = ler_int();
        if op == 1 {
            // menu do paciente
            loop {
                println!("Insira uma opção:\n1 - Login\n2 - Registro\n3 - Sair");
                op = ler_int();
        
                
                if op == 1 {
                    // Login
                    println!("Digite seu CPF:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
                    let cpf = input.trim().to_string();
                    input.clear();
        
                    println!("Digite sua senha:");
                    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
                    let senha = input.trim().to_string();
        
                    match database.login_paciente(&cpf, &senha).await {
                        Ok(Some(paciente)) => {
                            println!("Login bem-sucedido! Bem-vindo, {}.", paciente.get_nome());
                            paciente_logado_id = Some(paciente.get_id());
                            login = true;
                            break;
                        }
                        Ok(None) => {
                            println!("Erro: CPF ou senha inválidos!");
                        }
                        Err(e) => {
                            println!("Erro ao buscar paciente no banco: {}", e);
                        }
                    }
                }else if op == 2 {
                    // Registro
                    println!("Insira seus dados: ");
                    match paciente::Paciente::registro() {
                        Ok(paciente) => {
                            if pacientes.iter().any(|p| p.get_cpf() == paciente.get_cpf()) {
                                println!("Erro: Já existe um paciente cadastrado com este CPF!");
                            } else {
                                if let Err(e) = database.inserir_paciente(&paciente).await {
                                    println!("Erro ao cadastrar paciente no banco de dados: {}", e);
                                } else {
                                    println!("\nPaciente cadastrado com sucesso!");
                                    println!("CPF: {}", paciente.get_cpf());
                                    println!("Nome: {}", paciente.get_nome());
                                    println!("Data de Nascimento: {}", paciente.get_data_nascimento());
                                    println!("Senha: {}", paciente.get_senha());
                                    pacientes.push(paciente);
                                }
                            }
                        }
                        Err(e) => println!("Erro ao criar paciente: {}", e),
                    }
                }else if op == 4 {
                    println!("\nLista de Pacientes Cadastrados:");
                    for (i, paciente) in pacientes.iter().enumerate() {
                        println!(
                            "{}. CPF: {}, Nome: {}, Data de Nascimento: {}",
                            i + 1,
                            paciente.get_cpf(),
                            paciente.get_nome(),
                            paciente.get_data_nascimento()
                        );
                }
                }
                else if op == 3{
                    println!("Saindo do sistema...");
                    break;
                }
                else{
                    println!("Opção inválida! Tente novamente.");
                    continue;
                }
            }
            break;
        } else if op == 2 {
            
            // Login de médico
            if let Some(medico_logado) = Medico::login_medico(&medicos) {
                medico_logado_id = Some(medico_logado.get_id()); // <-- ADICIONE ESTA LINHA
                loop{
                println!("Escolha uma opção:\n1 - Listar consultas\n2 - Editar consulta\n3 - Sair");
                op = ler_int();
                if op == 1 {
                    if let Some(medico_id) = medico_logado_id {
                        // Buscar consultas do médico no banco
                        match database.buscar_consultas_medico(medico_id).await {
                            Ok(consultas_do_medico) => {
                                if consultas_do_medico.is_empty() {
                                    println!("Nenhuma consulta agendada para este médico.");
                                    continue; // Volta para o menu de opções do médico
                                }
                                consulta::Consulta::listar_consultas_medico(&consultas_do_medico, medico_id);
                            }
                            Err(e) => {
                                println!("Erro ao buscar consultas do médico no banco: {}", e);
                                continue;
                            }
                        }
                    } else {
                        println!("Nenhum médico logado.");
                    }
                    continue;
                } else if op == 2 {
                    println!("Digite o ID da consulta que deseja editar:");
                    if let Some(medico_id) = medico_logado_id {
                        // Buscar consultas do médico no banco
                        match database.buscar_consultas_medico(medico_id).await {
                            Ok(consultas_do_medico) => {
                                if consultas_do_medico.is_empty() {
                                    println!("Nenhuma consulta agendada para este médico.");
                                    continue; // Volta para o menu de opções do médico
                                }
                                consulta::Consulta::listar_consultas_medico(&consultas_do_medico, medico_id);
                            }
                            Err(e) => {
                                println!("Erro ao buscar consultas do médico no banco: {}", e);
                                continue;
                            }
                        }
                    } else {
                        println!("Nenhum médico logado.");
                    }
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
                    let id_consulta: i32 = match input.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            println!("ID inválido!");
                            continue;
                        }
                    };
                
                    println!("O que deseja editar?\n1 - Data\n2 - Hora\n");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
                    let op = input.trim().parse::<i32>().unwrap_or(0);
                    match op {
                        1 => {
                            println!("Digite a nova data da consulta (YYYY-MM-DD):");
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
                            let data: NaiveDate = match input.trim().parse() {
                                Ok(data) => data,
                                Err(_) => {
                                    println!("Data inválida!");
                                    continue;
                                }
                            };
                            match database.atualizar_data_consulta(id_consulta, data).await {
                                Ok(_) => println!("Data da consulta atualizada com sucesso!"),
                                Err(e) => println!("Erro ao atualizar data da consulta: {}", e),
                            }
                        continue;
                        }
                        2 => {
                            println!("Digite a nova hora da consulta (HH:MM):");
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
                            let hora: chrono::NaiveTime = match input.trim().parse() {
                                Ok(hora) => hora,
                                Err(_) => {
                                    println!("Hora inválida!");
                                    continue;
                                }
                            };
                            match database.atualizar_hora_consulta(id_consulta, hora).await {
                                Ok(_) => println!("Hora da consulta atualizada com sucesso!"),
                                Err(e) => println!("Erro ao atualizar hora da consulta: {}", e),
                            }
                            continue;
                        }
                        _ => println!("Opção inválida!"),
                    }
                } else if op == 3{
                    println!("Voltando ao menu principal...");
                    break;
                } else {
                    println!("Opção inválida! Tente novamente.");
                    continue;
                }
                
            }       
        }
        } else if op == 3 {
            println!("Saindo do sistema...");
            break;
        }
        else {
            println!("Opção inválida! Tente novamente.");
            continue;
        }
    }

    if login == true {
        loop {
            println!("Escolha uma opção:\n1 - Listar consultas\n2 - Marcar consulta\n3 - Sair");
            op = ler_int();
            if op == 1 {
                // Listar consultas
                if let Some(paciente_id) = paciente_logado_id {
                    match database.buscar_consultas_paciente(paciente_id).await {
                        Ok(consultas_do_paciente) => {
                            consulta::Consulta::listar_consultas(&consultas_do_paciente, paciente_id);
                            if consultas_do_paciente.is_empty() {
                                // Não mostra opção de cancelar se não há consultas
                                continue;
                            }
                            println!("Deseja cancelar uma consulta? (s/n)");
                            let mut confirmacao = String::new();
                            io::stdin()
                                .read_line(&mut confirmacao)
                                .expect("Erro ao ler entrada");
                            if confirmacao.trim().to_lowercase() == "s" {
                                println!("Digite o ID da consulta que deseja cancelar:");
                                let mut input = String::new();
                                io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
                                let id_consulta: i32 = match input.trim().parse() {
                                    Ok(id) => id,
                                    Err(_) => {
                                        println!("ID inválido!");
                                        continue;
                                    }
                                };
                                // Aqui você chama o método para remover do banco!
                                match database.deletar_consulta(id_consulta).await {
                                    Ok(_) => println!("Consulta cancelada com sucesso!"),
                                    Err(e) => println!("Erro ao cancelar consulta: {}", e),
                                }
                            } else {
                                println!("Nenhuma consulta cancelada.");
                            }
                        }
                        Err(e) => println!("Erro ao buscar consultas no banco: {}", e),
                    }
                } else {
                    println!("Nenhum paciente logado.");
                }
            }else if op == 2 {
                // Marcar consulta
                println!("Doutores disponíveis:");
                for medico in &medicos {
                    println!(
                        "ID: {}, Nome: {}, Especialidade: {}",
                        medico.get_id(),
                        medico.get_nome(),
                        medico.get_especialidade()
                    );
                }
                
                println!("Digite o ID do médico com quem deseja agendar a consulta:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
                let medico_id: i32 = match input.trim().parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("ID inválido!");
                        continue;
                    }
                };

                let medico_escolhido = match medicos.iter().find(|m| m.get_id() == medico_id) {
                    Some(medico) => medico,
                    None => {
                        println!("Erro: Médico com ID {} não encontrado!", medico_id);
                        continue;
                    }
                };
                
                println!(
                    "Você escolheu o médico: {}, Especialidade: {}. Confirmar ? (s/n)",
                    medico_escolhido.get_nome(),
                    medico_escolhido.get_especialidade()
                );

                let mut confirmacao = String::new();
                io::stdin()
                    .read_line(&mut confirmacao)
                    .expect("Erro ao ler entrada");
                if confirmacao.trim().to_lowercase() != "s" {
                    println!("Consulta não agendada.");
                    continue;
                }

                println!("Digite a data da consulta (YYYY-MM-DD):");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
                let data: NaiveDate = match input.trim().parse() {
                    Ok(data) => data,
                    Err(_) => {
                        println!("Data inválida!");
                        continue;
                    }
                };
                input.clear();

                println!("Digite a hora da consulta (HH:MM):");
                io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
                let hora: chrono::NaiveTime = match input.trim().parse() {
                    Ok(hora) => hora,
                    Err(_) => {
                        println!("Hora inválida!");
                        continue;
                    }
                };
                input.clear();

                println!("Médico escolhido: {}, Especialidade: {}", medico_escolhido.get_nome(), medico_escolhido.get_especialidade());
                println!("Data da consulta: {}", data);
                println!("Hora da consulta: {}", hora);
                println!("Confirmar agendamento? (s/n)");
                let mut confirmacao = String::new();
                io::stdin()
                    .read_line(&mut confirmacao)
                    .expect("Erro ao ler entrada");
                if confirmacao.trim().to_lowercase() != "s" {
                    println!("Consulta não agendada.");
                    continue;
                }   
                else {
                    if let Some(paciente_id) = paciente_logado_id {
                        // Busque o paciente no banco pelo ID
                        match database.buscar_paciente_por_id(paciente_id).await {
                            Ok(Some(paciente)) => {
                                println!("Consulta agendada com sucesso!");

                                let id_consulta = consultas.len() as i32 + 1;
                                let nova_consulta = consulta::Consulta::new(
                                    id_consulta,
                                    data,
                                    hora,
                                    paciente.get_id(),         // correto: id do paciente
                                    medico_escolhido.get_id(), // correto: id do médico
                                );

                                // Salva no banco
                                if let Err(e) = database.inserir_consulta(&nova_consulta).await {
                                    println!("Erro ao salvar consulta no banco: {}", e);
                                } else {
                                    consultas.push(nova_consulta);
                                }
                            }
                            Ok(None) => {
                                println!("Paciente não encontrado no banco!");
                            }
                            Err(e) => {
                                println!("Erro ao buscar paciente no banco: {}", e);
                            }
                        }
                    }
                }
            }else if op == 3 {
                println!("Saindo do sistema...");
                break;
            }
            else {
                println!("Opção inválida! Tente novamente.");
                continue;
            }
        }
    }

    Ok(())
}