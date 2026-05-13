use oracle::{Connection, Result};
use crate::orm::dto::library::Library;

pub struct LibraryDao<'a> {
    conn: &'a Connection,
}

impl<'a> LibraryDao<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn exists(&self, user_id: i32, game_id: i32) -> Result<bool> {
        let sql = "SELECT COUNT(*) FROM Library WHERE user_id = :1 AND game_id = :2";
        let count: i32 = self.conn.query_row_as(sql, &[&user_id, &game_id])?;
        Ok(count > 0)
    }

    pub fn insert(&self, library: &Library) -> Result<()> {
        let sql = "INSERT INTO Library (user_id, game_id, purchase_price) VALUES (:1, :2, :3)";
        self.conn.execute(sql, &[&library.user_id, &library.game_id, &library.purchase_price])?;
        Ok(())
    }

    pub fn get_game_id(&self, library_id: i32, user_id: i32) -> Result<Option<i32>> {
        let sql = "SELECT game_id FROM Library WHERE library_id = :1 AND user_id = :2";
        match self.conn.query_row_as::<i32>(sql, &[&library_id, &user_id]) {
            Ok(id) => Ok(Some(id)),
            Err(e) => {
                if e.kind() == oracle::ErrorKind::NoDataFound {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }
}
