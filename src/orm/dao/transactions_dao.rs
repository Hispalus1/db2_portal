#![allow(dead_code)]

use oracle::{Connection, Result};
use crate::orm::dao::app_user_dao::AppUserDao;
use crate::orm::dao::game_dao::GameDao;
use crate::orm::dao::library_dao::LibraryDao;
use crate::orm::dao::review_dao::ReviewDao;
use crate::orm::dao::wallet_transaction_dao::WalletTransactionDao;
use crate::orm::dto::library::Library;
use crate::orm::dto::review::Review;
use crate::orm::dto::wallet_transaction::WalletTransaction;

pub struct TransactionsDao<'a> {
    conn: &'a Connection,
}

impl<'a> TransactionsDao<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ==========================================
    // F7. BuyGame (Nákup hry)
    // ==========================================

    pub fn buy_game(
        &self,
        p_user_id: i32,
        p_selected_game_id: i32,
        p_is_gift: bool,
        p_recipient_user_id: Option<i32>,
    ) -> Result<bool> {
        let _ = self.conn.rollback();
        self.conn.execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE", &[])?;

        let user_dao = AppUserDao::new(self.conn);
        let game_dao = GameDao::new(self.conn);
        let library_dao = LibraryDao::new(self.conn);

        let buyer = match user_dao.get_by_id(p_user_id)? {
            Some(u) => u,
            None => {
                self.conn.rollback()?;
                return Ok(false);
            }
        };

        let game = match game_dao.get_by_id(p_selected_game_id)? {
            Some(g) => g,
            None => {
                self.conn.rollback()?;
                return Ok(false);
            }
        };

        let target_user_id = if p_is_gift { p_recipient_user_id.expect("Recipient ID must be provided for gifts") } else { p_user_id };

        if library_dao.exists(target_user_id, p_selected_game_id)? {
            self.conn.rollback()?;
            return Ok(false);
        }

        if buyer.wallet_balance < game.price {
            self.conn.rollback()?;
            return Ok(false);
        }

        let lib_entry = Library {
            library_id: 0,
            user_id: target_user_id,
            game_id: p_selected_game_id,
            purchase_price: game.price,
        };

        if let Err(e) = library_dao.insert(&lib_entry) {
            let _ = self.conn.rollback();
            return Err(e);
        }

        if let Err(e) = user_dao.update_balance(p_user_id, -game.price) {
            let _ = self.conn.rollback();
            return Err(e);
        }

        self.conn.commit()?;
        Ok(true)
    }

    pub fn buy_game_sp(
        &self,
        p_user_id: i32,
        p_selected_game_id: i32,
        p_is_gift: bool,
        p_recipient_user_id: Option<i32>,
    ) -> Result<bool> {
        let is_gift_int = if p_is_gift { 1i32 } else { 0i32 };
        
        let sql = "
            DECLARE
                v_ret NUMBER;
            BEGIN
                BuyGame_SP(:1, :2, :3, :4, v_ret);
                :5 := v_ret;
            END;
        ";

        let mut stmt = self.conn.statement(sql).build()?;

        stmt.execute(&[
            &p_user_id,
            &p_selected_game_id,
            &is_gift_int,
            &p_recipient_user_id,
            &oracle::sql_type::OracleType::Number(0, 0), 
        ])?;

        let out_ret: i32 = stmt.bind_value(5)?;

        Ok(out_ret == 1)
    }

    // ==========================================
    // F8. TopUpWallet (Dobití peněženky)
    // ==========================================
    pub fn top_up_wallet(&self, p_user_id: i32, p_amount: f64) -> Result<bool> {
        let _ = self.conn.rollback(); 
        self.conn.execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE", &[])?;

        let user_dao = AppUserDao::new(self.conn);
        let tx_dao = WalletTransactionDao::new(self.conn);

        let tx = WalletTransaction {
            transaction_id: 0,
            user_id: p_user_id,
            amount: p_amount,
        };

        if let Err(e) = tx_dao.insert(&tx) {
            let _ = self.conn.rollback();
            return Err(e);
        }

        if let Err(e) = user_dao.update_balance(p_user_id, p_amount) {
            let _ = self.conn.rollback();
            return Err(e);
        }

        self.conn.commit()?;
        Ok(true)
    }


    // ==========================================
    // F11. SaveReview (Uložení recenze)
    // ==========================================

    pub fn save_review(
        &self,
        p_user_id: i32,
        p_library_game_id: i32,
        p_rating: i32,
        p_comment: &str,
    ) -> Result<bool> {
        let _ = self.conn.rollback();
        self.conn.execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE", &[])?;

        let library_dao = LibraryDao::new(self.conn);
        let review_dao = ReviewDao::new(self.conn);

        let game_id = match library_dao.get_game_id(p_library_game_id, p_user_id)? {
            Some(id) => id,
            None => {
                self.conn.rollback()?;
                return Ok(false);
            }
        };

        let existing_review = review_dao.get_by_user_and_game(p_user_id, game_id)?;

        if let Some(mut rev) = existing_review {
            rev.rating = p_rating;
            rev.review_comment = p_comment.to_string();
            if let Err(e) = review_dao.update(&rev) {
                let _ = self.conn.rollback();
                return Err(e);
            }
        } else {
            let new_rev = Review {
                review_id: 0,
                user_id: p_user_id,
                game_id,
                rating: p_rating,
                review_comment: p_comment.to_string(),
            };
            if let Err(e) = review_dao.insert(&new_rev) {
                let _ = self.conn.rollback();
                return Err(e);
            }
        }

        self.conn.commit()?;
        Ok(true)
    }

    pub fn save_review_sp(
        &self,
        p_user_id: i32,
        p_library_game_id: i32,
        p_rating: i32,
        p_comment: &str,
    ) -> Result<bool> {
        let sql = "
            DECLARE
                v_ret NUMBER;
            BEGIN
                SaveReview_SP(:1, :2, :3, :4, v_ret);
                :5 := v_ret;
            END;
        ";

        let mut stmt = self.conn.statement(sql).build()?;

        stmt.execute(&[
            &p_user_id,
            &p_library_game_id,
            &p_rating,
            &p_comment,
            &oracle::sql_type::OracleType::Number(0, 0), 
        ])?;

        let out_ret: i32 = stmt.bind_value(5)?;

        Ok(out_ret == 1)
    }
}
