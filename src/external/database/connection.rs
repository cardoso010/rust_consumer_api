use rusqlite::{Connection, Result};

pub fn get_connection() -> Result<Connection> {
    let path = "./rust_consumer_api.db3";

    let db = Connection::open(path)?;

    Ok(db)
}
