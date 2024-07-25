use chrono::Utc;
use log::info;
use rusqlite::{params, Connection};
use std::error::Error;

use super::APYHistoryRecordAggr;
use super::SavingsAccountAggr;

pub struct APYHistoryRecordRepo<'a> {
    conn: &'a Connection,
}

impl<'a> APYHistoryRecordRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(
        &self,
        account: &SavingsAccountAggr,
        apy: f64,
    ) -> APYHistoryRecordAggr {

        APYHistoryRecordAggr::new(
            account.id(),
            apy,
            365,
            Utc::now().format("%Y-%m-%d").to_string(),
        )
    }

    pub fn commit(&self, apy_history_record: &APYHistoryRecordAggr) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO savings_accounts_apy_history (account_id, apy, compound_frequency, effective_date) VALUES (?1, ?2, ?3, ?4)";
        info!(
            "Committing APY history record to DB... Account ID: {}, APY: {}, Effective Date: {}, Compound Frequency: {}",
            apy_history_record.account_id(), apy_history_record.apy(), apy_history_record.effective_date(), apy_history_record.compound_frequency()
        );
        let params = params![
            apy_history_record.account_id(),
            apy_history_record.apy(),
            apy_history_record.compound_frequency(),
            apy_history_record.effective_date()
        ];
        self.conn.execute(query, params)?;

        Ok(())
    }
}
