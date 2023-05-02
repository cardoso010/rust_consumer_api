use rusqlite::Error;

use super::connection::get_connection;

pub fn execute() -> Result<usize, Error> {
    let conn = get_connection()?;

    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS chucknorris (
            id    TEXT PRIMARY KEY,
            icon_url  TEXT NOT NULL,
            url  TEXT NOT NULL,
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS adviceslip (
            id    INTEGER PRIMARY KEY,
            advice  TEXT NOT NULL
        );


    "#,
        (),
    )
}
