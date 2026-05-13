mod orm;

use oracle::{Connection, Result};
use orm::dao::transactions_dao::{buy_game, buy_game_sp, save_review, save_review_sp};

fn main() -> Result<()> {
    let conn = Connection::connect(
        "HRN0040", 
        "QiOGo354c52dLX0z", 
        "(DESCRIPTION=(ADDRESS=(PROTOCOL=TCP)(HOST=bayer.cs.vsb.cz)(PORT=1521))(CONNECT_DATA=(SID=oracle)))"
    )?;

    println!("Připojeno k databázi!\n");

    // ==========================================
    // F7: Nákup hry
    // ==========================================
    println!("--- F7: Nákup hry ---");
    
    // Test 1: RustEnjoyer (100) si zkusí koupit hru Rust (100).
    // Očekáváme FALSE, protože podle našich testovacích dat už ji vlastní.
    match buy_game(&conn, 100, 100, false, None) {
        Ok(ret) => println!("BuyGame (Rust):    ret: {}, user: 100, game: 100 (Správně zamítnuto - duplicita)", ret),
        Err(e) => println!("BuyGame (Rust) selhalo: {}", e),
    }

    // Test 2: GamerCz (1) si koupí novou hru GTA V (101). Peníze mu teď nechybí.
    // Očekáváme TRUE, protože tuhle hru ještě nemá.
    match buy_game_sp(&conn, 1, 101, false, None) {
        Ok(ret) => println!("BuyGame_SP (PLSQL):ret: {}, user: 1, game: 101 (Úspěšný nákup)", ret),
        Err(e) => println!("BuyGame_SP (PLSQL) selhalo: {}", e),
    }

    // ==========================================
    // F11: Uložení recenze
    // ==========================================
    println!("\n--- F11: Uložení recenze ---");
    
    // Test 3: RustEnjoyer (100) napíše novou recenzi na Garry's Mod (library_id = 101).
    match save_review(&conn, 100, 101, 10, "Nejlepší sandbox! (Zapsáno přes Rust)") {
        Ok(ret) => println!("SaveReview (Rust):    ret: {}, user: 100, lib_id: 101", ret),
        Err(e) => println!("SaveReview (Rust) selhalo: {}", e),
    }

    // Test 4: PL/SQL procedura tu samou recenzi obratem updatuje.
    match save_review_sp(&conn, 100, 101, 9, "Upraveno přes PL/SQL proceduru!") {
        Ok(ret) => println!("SaveReview_SP (PLSQL):ret: {}, user: 100, lib_id: 101", ret),
        Err(e) => println!("SaveReview_SP (PLSQL) selhalo: {}", e),
    }

    Ok(())
}