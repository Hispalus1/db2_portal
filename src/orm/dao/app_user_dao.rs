use oracle::{Connection, Result};
use crate::orm::dto::app_user::AppUser;

pub struct AppUserDao<'a> {
    conn: &'a Connection,
}

impl<'a> AppUserDao<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_by_id(&self, user_id: i32) -> Result<Option<AppUser>> {
        let sql = "SELECT user_id, username, wallet_balance FROM AppUser WHERE user_id = :1";
        match self.conn.query_row(sql, &[&user_id]) {
            Ok(row) => {
                Ok(Some(AppUser {
                    user_id: row.get(0)?,
                    username: row.get(1)?,
                    wallet_balance: row.get(2)?,
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
