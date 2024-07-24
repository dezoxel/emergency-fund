use rusqlite::{params_from_iter, Connection, Error};
use url::Url;

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
                let id: i32 = row.get(0)?;
                let url_str: Option<String> = row.get(1)?;

                let terms_and_conditions_source_url = match url_str {
                    Some(ref url) => Url::parse(url).ok(),
                    None => None,
                };

                Ok(SavingsAccountAggr::new(id, terms_and_conditions_source_url))
            })?
            .collect::<Result<Vec<SavingsAccountAggr>, Error>>()?;

        Ok(accounts)
    }
}
