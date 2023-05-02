use rusqlite::{Error, Result};

use super::repository::Repository;
use crate::external::database::connection::get_connection;

#[derive(Debug)]
pub struct Adviceslip {
    id: i64,
    advice: String,
}

pub struct AdviceslipRepository;

impl Repository<Adviceslip> for AdviceslipRepository {
    fn all(self) -> Result<Vec<Adviceslip>, Error> {
        let conn = get_connection()?;

        let mut stmt = conn.prepare("SELECT id, advice FROM adviceslip")?;
        let chuck_norris_iter = stmt.query_map([], |row| {
            Ok(Adviceslip {
                id: row.get(0)?,
                advice: row.get(1)?,
            })
        })?;

        let mut chucks = Vec::new();

        for chuck in chuck_norris_iter {
            chucks.push(chuck.unwrap());
        }

        Ok(chucks)
    }

    fn get(self, id: String) -> Result<Adviceslip, Error> {
        let conn = get_connection()?;

        let mut stmt = conn.prepare("SELECT id, advice FROM adviceslip where id = :id")?;
        let chuck_norris = stmt
            .query_map(&[(":id", &id)], |row| {
                Ok(Adviceslip {
                    id: row.get(0)?,
                    advice: row.get(1)?,
                })
            })?
            .last();

        match chuck_norris {
            Some(chuck) => chuck,
            None => Err(Error::QueryReturnedNoRows),
        }
    }

    fn save(self, chuck: &Adviceslip) -> Result<usize> {
        let conn = get_connection()?;

        conn.execute(
            "INSERT INTO adviceslip (id, advice) VALUES (?1, ?2)",
            (&chuck.id, &chuck.advice),
        )
    }

    fn delete(self, id: String) -> Result<usize> {
        let conn = get_connection()?;

        conn.execute("DELETE FROM adviceslip WHERE id = ?", [&id])
    }
}
