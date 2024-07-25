use std::error::Error;
use rusqlite::{params, Connection, Result};
use chrono::Utc;

pub fn write_apy_to_db(conn: &Connection, account_id: i32, apy: f64) -> Result<(), Box<dyn Error>> {
    println!(
        "Writing APY to DB... Account ID: {}, APY: {}",
        account_id, apy
    );
    let query = "INSERT INTO savings_accounts_apy_history (account_id, apy, compound_frequency, effective_date) VALUES (?1, ?2, ?3, ?4)";
    let compound_frequency = 365;
    let effective_date = Utc::now().format("%Y-%m-%d").to_string();
    let params = params![account_id, apy, compound_frequency, effective_date];
    conn.execute(query, params)?;
    println!("APY is written to the database");

    Ok(())
}
