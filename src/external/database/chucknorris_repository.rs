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

fn get_connection() -> Result<Connection> {
    Connection::open_in_memory()
}

pub fn setup() -> Result<()> {
    let conn = get_connection()?;

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

pub fn all() -> Result<Vec<ChuckNorris>> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare("SELECT id, icon_url, url, created_at, updated_at FROM chucknorris")?;
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

pub fn get_by_id(id: i32) -> Result<Vec<ChuckNorris>> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare("SELECT id, icon_url, url, created_at, updated_at FROM chucknorris where id = :id")?;
    let chuck_norris_iter = stmt.query_map(&[(":id", &id)], |row| {
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

pub fn insert(chuck: ChuckNorris) -> Result<usize> {
    let conn = get_connection()?;

    conn.execute(
        "INSERT INTO person (icon_url, url, created_at, updated_at) VALUES (?1, ?2)",
        (&chuck.icon_url, &chuck.url, &chuck.created_at, &chuck.updated_at),
    )
}


