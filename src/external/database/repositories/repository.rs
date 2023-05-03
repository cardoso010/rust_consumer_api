use rusqlite::{Error, Result};

pub trait Repository<T> {
    fn all() -> Result<Vec<T>, Error>;
    fn get(id: String) -> Result<T, Error>;
    fn save(data: &T) -> Result<usize>;
    fn delete(id: String) -> Result<usize>;
}
