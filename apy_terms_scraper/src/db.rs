use std::error::Error;
use rusqlite::{params, params_from_iter, Connection, Result};
use chrono::Utc;
use std::collections::HashMap;

pub fn write_apy_to_db(conn: &Connection, account_id: i32, apy: f32) -> Result<(), Box<dyn Error>> {
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

pub fn fetch_institution_names_by_account_ids(
    ids: &Vec<i32>,
    conn: &Connection,
) -> Result<HashMap<i32, String>, Box<dyn Error>> {
    let query = format!(
        r#"
SELECT sa.id, i.name
FROM savings_accounts sa
JOIN institutions i ON sa.institution_id = i.id
WHERE sa.id IN ({})
"#,
        ids.iter().map(|_| "?").collect::<Vec<_>>().join(",")
    );

    let mut stmt = conn.prepare(&query)?;
    let institution_names_iter = stmt.query_map(params_from_iter(ids.iter()), |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, Option<String>>(1)?))
    })?;

    let mut institution_names = HashMap::new();
    for result in institution_names_iter {
        if let Ok((id, Some(name))) = result {
            institution_names.insert(id, name);
        }
    }

    Ok(institution_names)
}
