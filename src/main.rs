mod orm;

use oracle::{Connection, Result};
use orm::dao::transactions_dao::TransactionsDao;
use orm::dao::app_user_dao::AppUserDao;
use orm::dao::game_dao::GameDao;

fn main() -> Result<()> {
    let conn = Connection::connect(
        "HRN0040",
        "QiOGo354c52dLX0z",
        "(DESCRIPTION=(ADDRESS=(PROTOCOL=TCP)(HOST=bayer.cs.vsb.cz)(PORT=1521))(CONNECT_DATA=(SID=oracle)))"
    )?;

    println!("Připojeno k databázi!\n");

    let tx_dao = TransactionsDao::new(&conn);
    let user_dao = AppUserDao::new(&conn);
    let game_dao = GameDao::new(&conn);

    println!("--- Debug: Výpis všech uživatelů ---");
    let mut stmt = conn.statement("SELECT user_id, username, wallet_balance FROM AppUser").build()?;
    let rows = stmt.query(&[])?;
    for row_result in rows {
        let row = row_result?;
        let user_id: i32 = row.get(0)?;
        let username: String = row.get(1)?;
        let balance: f64 = row.get(2)?;
        println!("ID: {}, Name: {}, Balance: {}", user_id, username, balance);
    }

    println!("\n--- Debug: Výpis všech her ---");
    let mut stmt = conn.statement("SELECT game_id, price FROM Game").build()?;
    let rows = stmt.query(&[])?;
    for row_result in rows {
        let row = row_result?;
        let game_id: i32 = row.get(0)?;
        let price: f64 = row.get(1)?;
        println!("ID: {}, Price: {}", game_id, price);
    }

    println!("\n--- Debug: Výpis knihovny (Library) ---");
    let mut stmt = conn.statement("SELECT user_id, game_id, purchase_price FROM Library").build()?;
    let rows = stmt.query(&[])?;
    for row_result in rows {
        let row = row_result?;
        let u_id: i32 = row.get(0)?;
        let g_id: i32 = row.get(1)?;
        let p: f64 = row.get(2)?;
        println!("User: {}, Game: {}, Price: {}", u_id, g_id, p);
    }
    println!("---------------------------------------\n");

    // Ukázka použití nových DAO s DTO
    if let Some(user) = user_dao.get_by_id(100)? {
        println!("Nalezen uživatel: {:?}", user);
    } else {
        println!("Uživatel s ID 100 nenalezen.");
    }
    if let Some(game) = game_dao.get_by_id(100)? {
        println!("Nalezena hra: {:?}", game);
    } else {
        println!("Hra s ID 100 nenalezena.");
    }
    println!("");

    // ==========================================
    // F7: Nákup hry
    // ==========================================
    println!("--- F7: Nákup hry ---");

    // Test 1: RustEnjoyer (100) si zkusí koupit hru Rust (100).
    // Očekáváme FALSE, protože podle našich testovacích dat už ji vlastní.
    match tx_dao.buy_game(100, 100, false, None) {
        Ok(ret) => println!("BuyGame (Rust):    ret: {}, user: 100, game: 100 (Správně zamítnuto - duplicita)", ret),
        Err(e) => println!("BuyGame (Rust) selhalo: {}", e),
    }

    // Test 2: GamerCz (1) si koupí novou hru ToxicPlayer_99 (102). Peníze mu teď nechybí.
    // Očekáváme TRUE, protože tuhle hru ještě nemá.
    match tx_dao.buy_game_sp(1, 102, false, None) {
        Ok(ret) => println!("BuyGame_SP (PLSQL):ret: {}, user: 1, game: 102 (Úspěšný nákup)", ret),
        Err(e) => println!("BuyGame_SP (PLSQL) selhalo: {}", e),
    }

    // ==========================================
    // F11: Uložení recenze
    // ==========================================
    println!("\n--- F11: Uložení recenze ---");

    // Test 3: RustEnjoyer (100) napíše novou recenzi na Garry's Mod (library_id = 101).
    match tx_dao.save_review(100, 101, 10, "Nejlepší sandbox! (Zapsáno přes Rust)") {
        Ok(ret) => println!("SaveReview (Rust):    ret: {}, user: 100, lib_id: 101", ret),
        Err(e) => println!("SaveReview (Rust) selhalo: {}", e),
    }

    // Test 4: PL/SQL procedura tu samou recenzi obratem updatuje.
    match tx_dao.save_review_sp(100, 101, 9, "Upraveno přes PL/SQL proceduru!") {
        Ok(ret) => println!("SaveReview_SP (PLSQL):ret: {}, user: 100, lib_id: 101", ret),
        Err(e) => println!("SaveReview_SP (PLSQL) selhalo: {}", e),
    }

    Ok(())
}
