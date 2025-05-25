use sqlx::mysql::MySqlPool;
use std::error::Error;
use chrono::{NaiveDate, NaiveTime};
use crate::paciente::Paciente;
use crate::medico::Medico;
use crate::consulta::Consulta;
use sqlx::Row;

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = MySqlPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn criar_tabelas(&self) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS pacientes (
                id INT PRIMARY KEY AUTO_INCREMENT,
                cpf VARCHAR(11) UNIQUE NOT NULL,
                nome VARCHAR(100) NOT NULL,
                data_nascimento DATE NOT NULL,
                senha VARCHAR(100) NOT NULL
            )"
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS medicos (
                id INT PRIMARY KEY AUTO_INCREMENT,
                nome VARCHAR(100) NOT NULL,
                data_nascimento DATE NOT NULL,
                senha VARCHAR(100) NOT NULL,
                crm VARCHAR(20) UNIQUE NOT NULL,
                especialidade VARCHAR(50) NOT NULL
            )"
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS consultas (
                id_consulta INT PRIMARY KEY AUTO_INCREMENT,
                data DATE NOT NULL,
                hora TIME NOT NULL,
                paciente_id INT NOT NULL,
                medico_id INT NOT NULL,
                FOREIGN KEY (paciente_id) REFERENCES pacientes(id),
                FOREIGN KEY (medico_id) REFERENCES medicos(id)
            )"
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn buscar_pacientes(&self) -> Result<Vec<Paciente>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT id, cpf, nome, data_nascimento, senha FROM pacientes"
        )
        .fetch_all(&self.pool)
        .await?;

        let pacientes = rows.into_iter().map(|row| {
            Paciente::new(
                row.get("cpf"),
                row.get("nome"),
                row.get("data_nascimento"),
                row.get("senha"),
                row.get("id"),
            )
        }).collect();

        Ok(pacientes)
    }

    pub async fn buscar_medicos(&self) -> Result<Vec<Medico>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT id, nome, data_nascimento, senha, crm, especialidade FROM medicos"
        )
        .fetch_all(&self.pool)
        .await?;

        let medicos = rows.into_iter().map(|row| {
            Medico::new(
                row.get("nome"),
                row.get("data_nascimento"),
                row.get("senha"),
                row.get("crm"),
                row.get("especialidade"),
                row.get("id"),
            )
        }).collect();

        Ok(medicos)
    }

    pub async fn inserir_paciente(&self, paciente: &Paciente) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "INSERT INTO pacientes (cpf, nome, data_nascimento, senha) VALUES (?, ?, ?, ?)"
        )
        .bind(paciente.get_cpf())
        .bind(paciente.get_nome())
        .bind(paciente.get_data_nascimento())
        .bind(paciente.get_senha())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn inserir_medico(&self, medico: &Medico) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "INSERT INTO medicos (nome, data_nascimento, senha, crm, especialidade) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(medico.get_nome())
        .bind(medico.get_data_nascimento())
        .bind(medico.get_senha())
        .bind(medico.get_crm())
        .bind(medico.get_especialidade())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn inserir_consulta(&self, consulta: &Consulta) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "INSERT INTO consultas (data, hora, paciente_id, medico_id) VALUES (?, ?, ?, ?)"
        )
        .bind(consulta.data)
        .bind(consulta.hora)
        .bind(consulta.get_id_paciente())
        .bind(consulta.get_id_medico())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn login_paciente(
        &self,
        cpf: &str,
        senha: &str,
    ) -> Result<Option<Paciente>, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT id, cpf, nome, data_nascimento, senha FROM pacientes WHERE cpf = ? AND senha = ?"
        )
        .bind(cpf)
        .bind(senha)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Paciente::new(
                row.get("cpf"),
                row.get("nome"),
                row.get("data_nascimento"),
                row.get("senha"),
                row.get("id"),
            )))
        } else {
            Ok(None)
        }
    }

    pub async fn buscar_paciente_por_id(
        &self,
        id: i32,
    ) -> Result<Option<Paciente>, Box<dyn std::error::Error>> {
        let row = sqlx::query(
            "SELECT id, cpf, nome, data_nascimento, senha FROM pacientes WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Paciente::new(
                row.get("cpf"),
                row.get("nome"),
                row.get("data_nascimento"),
                row.get("senha"),
                row.get("id"),
            )))
        } else {
            Ok(None)
        }
    }

    pub async fn buscar_consultas(&self) -> Result<Vec<Consulta>, Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            "SELECT id_consulta, data, hora, paciente_id, medico_id FROM consultas"
        )
        .fetch_all(&self.pool)
        .await?;

        let consultas = rows.into_iter().map(|row| {
            Consulta::new(
                row.get("id_consulta"),
                row.get("data"),
                row.get("hora"),
                row.get("paciente_id"),
                row.get("medico_id"),
            )
        }).collect();

        Ok(consultas)
    }


    pub async fn buscar_consultas_paciente(&self, paciente_id: i32) -> Result<Vec<Consulta>, Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            "SELECT id_consulta, data, hora, paciente_id, medico_id FROM consultas WHERE paciente_id = ?"
        )
        .bind(paciente_id)
        .fetch_all(&self.pool)
        .await?;

        let consultas = rows.into_iter().map(|row| {
            Consulta::new(
                row.get("id_consulta"),
                row.get("data"),
                row.get("hora"),
                row.get("paciente_id"),
                row.get("medico_id"),
            )
        }).collect();

        Ok(consultas)
    }

    pub async fn buscar_consultas_medico(&self, medico_id: i32) -> Result<Vec<Consulta>, Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            "SELECT id_consulta, data, hora, paciente_id, medico_id FROM consultas WHERE medico_id = ?"
        )
        .bind(medico_id)
        .fetch_all(&self.pool)
        .await?;

        let consultas = rows.into_iter().map(|row| {
            Consulta::new(
                row.get("id_consulta"),
                row.get("data"),
                row.get("hora"),
                row.get("paciente_id"),
                row.get("medico_id"),
            )
        }).collect();

        Ok(consultas)
    }

    pub async fn deletar_consulta(&self, id_consulta: i32) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("DELETE FROM consultas WHERE id_consulta = ?")
            .bind(id_consulta)
            .execute(&self.pool)
            .await?;
        Ok(())
    }


    // Atualizar data da consulta
    pub async fn atualizar_data_consulta(&self, id_consulta: i32, nova_data: NaiveDate) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("UPDATE consultas SET data = ? WHERE id_consulta = ?")
            .bind(nova_data)
            .bind(id_consulta)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Atualizar hora da consulta
    pub async fn atualizar_hora_consulta(&self, id_consulta: i32, nova_hora: NaiveTime) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("UPDATE consultas SET hora = ? WHERE id_consulta = ?")
            .bind(nova_hora)
            .bind(id_consulta)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
