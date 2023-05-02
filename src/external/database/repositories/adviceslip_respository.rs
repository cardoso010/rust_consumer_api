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
        let adviceslip_iter = stmt.query_map([], |row| {
            Ok(Adviceslip {
                id: row.get(0)?,
                advice: row.get(1)?,
            })
        })?;

        let mut adviceslips = Vec::new();

        for adviceslip in adviceslip_iter {
            adviceslips.push(adviceslip.unwrap());
        }

        Ok(adviceslips)
    }

    fn get(self, id: String) -> Result<Adviceslip, Error> {
        let conn = get_connection()?;

        let mut stmt = conn.prepare("SELECT id, advice FROM adviceslip where id = :id")?;
        let adviceslip = stmt
            .query_map(&[(":id", &id)], |row| {
                Ok(Adviceslip {
                    id: row.get(0)?,
                    advice: row.get(1)?,
                })
            })?
            .last();

        match adviceslip {
            Some(advice) => advice,
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
