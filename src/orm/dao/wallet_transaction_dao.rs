use oracle::{Connection, Result};
use crate::orm::dto::wallet_transaction::WalletTransaction;

pub struct WalletTransactionDao<'a> {
    conn: &'a Connection,
}

impl<'a> WalletTransactionDao<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn insert(&self, tx: &WalletTransaction) -> Result<()> {
        let sql = "INSERT INTO WalletTransaction (user_id, amount) VALUES (:1, :2)";
        self.conn.execute(sql, &[&tx.user_id, &tx.amount])?;
        Ok(())
    }
}
