use rusqlite::{params_from_iter, Connection, Error};

use super::SavingsAccountAggr;

pub struct SavingsAccountRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SavingsAccountRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn find_by_ids(&self, account_ids: Vec<i32>) -> Result<Vec<SavingsAccountAggr>, Error> {
        let placeholders = account_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");
        let query = format!(
            "SELECT id, terms_and_conditions_source_url FROM savings_accounts WHERE id IN ({})",
            placeholders
        );
        let mut stmt = self.conn.prepare(&query)?;
        let accounts = stmt
            .query_map(params_from_iter(account_ids.iter()), |row| {
                Ok(SavingsAccountAggr::new(
                    row.get(0)?,
                    row.get(1)?,
                ))
            })?
            .collect::<Result<Vec<SavingsAccountAggr>, Error>>()?;

        Ok(accounts)
    }
}
