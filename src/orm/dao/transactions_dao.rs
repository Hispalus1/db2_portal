#![allow(deprecated)]
#![allow(dead_code)]

use oracle::{Connection, Result};

// ==========================================
// F7. BuyGame (Nákup hry)
// ==========================================

pub fn buy_game(
    conn: &Connection,
    p_user_id: i32,
    p_selected_game_id: i32,
    p_is_gift: bool,
    p_recipient_user_id: Option<i32>,
) -> Result<bool> {
    let _ = conn.rollback();
    conn.execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE", &[])?;

    let target_user_id = if p_is_gift { p_recipient_user_id.unwrap() } else { p_user_id };

    let count: i32 = conn.query_row_as(
        "SELECT COUNT(*) FROM Library WHERE user_id = :1 AND game_id = :2",
        &[&target_user_id, &p_selected_game_id],
    )?;

    if count > 0 {
        conn.rollback()?;
        return Ok(false);
    }

    let sql_check = "
        SELECT g.price, u.wallet_balance 
        FROM Game g, AppUser u 
        WHERE g.game_id = :1 AND u.user_id = :2
    ";
    
    let row = match conn.query_row(sql_check, &[&p_selected_game_id, &p_user_id]) {
        Ok(r) => r,
        Err(_) => {
            conn.rollback()?;
            return Ok(false);
        }
    };

    let price: f64 = row.get(0)?;
    let balance: f64 = row.get(1)?;

    if balance < price {
        conn.rollback()?;
        return Ok(false);
    }

    if let Err(e) = conn.execute(
        "INSERT INTO Library (user_id, game_id, purchase_price) VALUES (:1, :2, :3)",
        &[&target_user_id, &p_selected_game_id, &price],
    ) {
        let _ = conn.rollback();
        return Err(e);
    }

    if let Err(e) = conn.execute(
        "UPDATE AppUser SET wallet_balance = wallet_balance - :1 WHERE user_id = :2",
        &[&price, &p_user_id],
    ) {
        let _ = conn.rollback();
        return Err(e);
    }

    conn.commit()?;
    Ok(true)
}

pub fn buy_game_sp(
    conn: &Connection,
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

    let mut stmt = conn.statement(sql).build()?;

    // OPRAVA: Přidány parametry (0, 0)
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
pub fn top_up_wallet(conn: &Connection, p_user_id: i32, p_amount: f64) -> Result<bool> {
    let _ = conn.rollback(); // Bezpečnostní reset
    conn.execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE", &[])?;

    // 1. Zápis do historie transakcí (INSERT)
    if let Err(e) = conn.execute(
        "INSERT INTO WalletTransaction (user_id, amount) VALUES (:1, :2)",
        &[&p_user_id, &p_amount],
    ) {
        let _ = conn.rollback();
        return Err(e);
    }

    // 2. Přičtení peněz k zůstatku (UPDATE)
    if let Err(e) = conn.execute(
        "UPDATE AppUser SET wallet_balance = wallet_balance + :1 WHERE user_id = :2",
        &[&p_amount, &p_user_id],
    ) {
        let _ = conn.rollback();
        return Err(e);
    }

    conn.commit()?;
    Ok(true)
}


// ==========================================
// F11. SaveReview (Uložení recenze)
// ==========================================

pub fn save_review(
    conn: &Connection,
    p_user_id: i32,
    p_library_game_id: i32,
    p_rating: i32,
    p_comment: &str,
) -> Result<bool> {
    let _ = conn.rollback();
    conn.execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE", &[])?;

    let sql_lib = "SELECT game_id FROM Library WHERE library_id = :1 AND user_id = :2";
    let game_id: i32 = match conn.query_row_as(sql_lib, &[&p_library_game_id, &p_user_id]) {
        Ok(id) => id,
        Err(_) => {
            conn.rollback()?;
            return Ok(false);
        }
    };

    let sql_rev = "SELECT review_id FROM Review WHERE user_id = :1 AND game_id = :2";
    let review_id: Option<i32> = match conn.query_row_as(sql_rev, &[&p_user_id, &game_id]) {
        Ok(id) => Some(id),
        Err(oracle::Error::NoDataFound) => None,
        Err(e) => {
            conn.rollback()?;
            return Err(e);
        }
    };

    if let Some(r_id) = review_id {
        if let Err(e) = conn.execute(
            "UPDATE Review SET rating = :1, review_comment = :2 WHERE review_id = :3",
            &[&p_rating, &p_comment, &r_id],
        ) {
            let _ = conn.rollback();
            return Err(e);
        }
    } else {
        if let Err(e) = conn.execute(
            "INSERT INTO Review (user_id, game_id, rating, review_comment) VALUES (:1, :2, :3, :4)",
            &[&p_user_id, &game_id, &p_rating, &p_comment],
        ) {
            let _ = conn.rollback();
            return Err(e);
        }
    }

    conn.commit()?;
    Ok(true)
}

pub fn save_review_sp(
    conn: &Connection,
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

    let mut stmt = conn.statement(sql).build()?;

    // OPRAVA: Přidány parametry (0, 0)
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