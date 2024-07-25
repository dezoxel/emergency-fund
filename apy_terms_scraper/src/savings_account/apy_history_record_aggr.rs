pub struct APYHistoryRecordAggr {
    account_id: i32,
    apy: f64,
    compound_frequency: i32,
    effective_date: String,
}

impl APYHistoryRecordAggr {
    pub fn new(
        account_id: i32,
        apy: f64,
        compound_frequency: i32,
        effective_date: String,
    ) -> Self {
        Self {
            account_id,
            apy,
            compound_frequency,
            effective_date,
        }
    }

    pub fn account_id(&self) -> i32 {
        self.account_id
    }

    pub fn apy(&self) -> f64 {
        self.apy
    }

    pub fn compound_frequency(&self) -> i32 {
        self.compound_frequency
    }
    
    pub fn effective_date(&self) -> &String {
        &self.effective_date
    }
}
