use rusqlite::{Error, Result};

pub trait Repository<T> {
    fn all(self) -> Result<Vec<T>, Error>;
    fn get(self, id: String) -> Result<T, Error>;
    fn save(self, data: &T) -> Result<usize>;
    fn delete(self, id: String) -> Result<usize>;
}
