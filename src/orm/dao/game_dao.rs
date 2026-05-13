use oracle::{Connection, Result};
use crate::orm::dto::game::Game;

pub struct GameDao<'a> {
    conn: &'a Connection,
}

impl<'a> GameDao<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_by_id(&self, game_id: i32) -> Result<Option<Game>> {
        let sql = "SELECT game_id, price FROM Game WHERE game_id = :1";
        match self.conn.query_row(sql, &[&game_id]) {
            Ok(row) => {
                Ok(Some(Game {
                    game_id: row.get(0)?,
                    price: row.get(1)?,
                }))
            }
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
