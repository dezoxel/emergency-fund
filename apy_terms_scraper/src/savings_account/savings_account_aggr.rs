pub struct SavingsAccountAggr {
    id: i32,
    terms_and_conditions_source_url: Option<String>,
}

impl SavingsAccountAggr {
    pub fn new(
        id: i32,
        terms_and_conditions_source_url: Option<String>,
    ) -> Self {
        Self {
            id,
            terms_and_conditions_source_url,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn terms_and_conditions_source_url(&self) -> Option<&String> {
        self.terms_and_conditions_source_url.as_ref()
    }
}
