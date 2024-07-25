use std::str::FromStr;

use rusqlite::{params_from_iter, Connection, Error};
use url::Url;

use crate::institution::InstitutionName;

use super::SavingsAccountAggr;

pub struct SavingsAccountRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SavingsAccountRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn find_by_ids(&self, account_ids: &Vec<i32>) -> Result<Vec<SavingsAccountAggr>, Error> {
        let placeholders = account_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");
        let query = format!(
            r#"
            SELECT sa.id, sa.terms_and_conditions_source_url, i.name as institution_name
            FROM savings_accounts sa
            JOIN institutions i ON sa.institution_id = i.id
            WHERE sa.id IN ({})
            "#,
            placeholders
        );
        let mut stmt = self.conn.prepare(&query)?;

        let accounts = stmt
            .query_map(params_from_iter(account_ids.iter()), |row| {
                let id: i32 = row.get(0)?;
                let url_raw: Option<String> = row.get(1)?;
                let institution_name_raw: String = row.get(2)?;

                let terms_and_conditions_source_url = match url_raw {
                    Some(ref url) => Some(Url::parse(url).map_err(|e| {
                        Error::UserFunctionError(Box::new(e))
                    })?),
                    None => None,
                };

                let institution_name =
                    InstitutionName::from_str(&institution_name_raw).map_err(|e| {
                        Error::UserFunctionError(Box::new(e))
                    })?;

                Ok(SavingsAccountAggr::new(
                    id,
                    terms_and_conditions_source_url,
                    institution_name,
                ))
            })?
            .collect::<Result<Vec<SavingsAccountAggr>, Error>>()?;

        Ok(accounts)
    }
}

