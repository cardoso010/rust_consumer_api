use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct ChuckNorris {
    categories: Vec<String>,
    created_at: String,
    icon_url: String,
    id: String,
    updated_at: String,
    url: String,
}

fn setup() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE chucknorris (
            id    TEXT PRIMARY KEY,
            icon_url  TEXT NOT NULL,
            url  TEXT NOT NULL,
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;

    Ok(())
}

fn all() -> Result<Vec<ChuckNorris>> {
    let conn = Connection::open_in_memory()?;

    let mut stmt = conn.prepare("SELECT id, icon_url, url, created_at, updated_at FROM chucknorris")?;
    let chuck_norris_iter = stmt.query_map([], |row| {
        Ok(ChuckNorris {
            categories: vec![],
            id: row.get(0)?,
            icon_url: row.get(1)?,
            url: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    let mut chucks = Vec::new();

    for chuck in chuck_norris_iter {
        chucks.push(chuck.unwrap());
    }

    Ok(chucks)
}
