use chrono::{NaiveDate, NaiveTime};
use std::io::{self, Write};

pub struct Consulta {
    pub id_consulta: i32,       // ID único da consulta
    pub data: NaiveDate,        // Data da consulta
    pub hora: NaiveTime,        // Hora da consulta
    pub id_paciente: i32,       // ID do paciente
    pub id_medico: i32,         // ID do médico
}

impl Consulta {
    pub fn new(
        id_consulta: i32,
        data: NaiveDate,
        hora: NaiveTime,
        id_paciente: i32,
        id_medico: i32,
    ) -> Self {
        Self {
            id_consulta,
            data,
            hora,
            id_paciente,
            id_medico,
        }
    }

    pub fn get_id_consulta(&self) -> i32 {
        self.id_consulta
    }

    pub fn get_data(&self) -> NaiveDate {
        self.data
    }

    pub fn get_hora(&self) -> NaiveTime {
        self.hora
    }

    pub fn get_id_paciente(&self) -> i32 {
        self.id_paciente
    }

    pub fn get_id_medico(&self) -> i32 {
        self.id_medico
    }

    pub fn mostrar_consultas(consultas: &Vec<Consulta>) {
        if consultas.is_empty() {
            println!("Nenhuma consulta agendada.");
        } else {
            println!("Consultas agendadas:");
            for consulta in consultas {
                println!(
                    "ID: {}, Data: {}, Hora: {}",
                    consulta.get_id_consulta(),
                    consulta.get_data(),
                    consulta.get_hora()
                );
            }
        }
    }

    pub fn listar_consultas(consultas: &Vec<Consulta>, paciente_id: i32) {
        let consultas_usuario: Vec<&Consulta> = consultas
            .iter()
            .filter(|c| c.id_paciente == paciente_id)
            .collect();

        if consultas_usuario.is_empty() {
            println!("Nenhuma consulta agendada para este usuário.");
        } else {
            println!("Consultas agendadas para o usuário:");
            for consulta in consultas_usuario {
                println!(
                    "ID: {}, Data: {}, Hora: {}",
                    consulta.get_id_consulta(),
                    consulta.get_data(),
                    consulta.get_hora()
                );
            }
        }
    }


    pub fn listar_consultas_medico(consultas: &Vec<Consulta>, medico_id: i32) {
        let consultas_medico: Vec<&Consulta> = consultas
            .iter()
            .filter(|c| c.id_medico == medico_id)
            .collect();

        if consultas_medico.is_empty() {
            println!("Nenhuma consulta agendada para este médico.");
            return;
        } else {
            println!("Consultas agendadas para o médico:");
            for consulta in consultas_medico {
                println!(
                    "ID: {}, Data: {}, Hora: {}, Paciente ID: {}",
                    consulta.get_id_consulta(),
                    consulta.get_data(),
                    consulta.get_hora(),
                    consulta.get_id_paciente()
                );
            }
        }
    }
}
