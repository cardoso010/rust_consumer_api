use rusqlite::{Error, Result};

use crate::domain::entities::chucknorris::ChuckNorris;
use crate::external::database::connection::get_connection;

fn all() -> Result<Vec<ChuckNorris>, Error> {
    let conn = get_connection()?;

    let mut stmt =
        conn.prepare("SELECT id, icon_url, url, created_at, updated_at FROM chucknorris")?;
    let chuck_norris_iter = stmt.query_map([], |row| {
        Ok(ChuckNorris {
            id: row.get(0)?,
            icon_url: row.get(1)?,
            url: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            categories: vec![],
        })
    })?;

    let mut chucks = Vec::new();

    for chuck in chuck_norris_iter {
        chucks.push(chuck.unwrap());
    }

    Ok(chucks)
}

fn get(id: String) -> Result<ChuckNorris, Error> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT id, icon_url, url, created_at, updated_at FROM chucknorris where id = :id",
    )?;
    let chuck_norris = stmt
        .query_map(&[(":id", &id)], |row| {
            Ok(ChuckNorris {
                id: row.get(0)?,
                icon_url: row.get(1)?,
                url: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                categories: vec![],
            })
        })?
        .last();

    match chuck_norris {
        Some(chuck) => chuck,
        None => Err(Error::QueryReturnedNoRows),
    }
}

pub fn save(data: ChuckNorris) -> Result<usize> {
    let conn = get_connection()?;

    conn.execute(
        "INSERT INTO chucknorris (id, icon_url, url, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &data.id,
            &data.icon_url,
            &data.url,
            &data.created_at,
            &data.updated_at,
        ),
    )
}

fn delete(id: String) -> Result<usize> {
    let conn = get_connection()?;

    conn.execute("DELETE FROM chucknorris WHERE id = ?", [&id])
}
