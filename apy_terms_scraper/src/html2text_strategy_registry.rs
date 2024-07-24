use std::error::Error;

use crate::institution::InstitutionName;
use crate::everbank_html2text_strategy::EverBankStrategy;
use crate::sofi_bank_html2text_strategy::SoFiBankStrategy;
use crate::html2text_strategy::{Strategy, StrategyFactory};

impl StrategyFactory {
    pub fn create(&self, name: &InstitutionName) -> Result<Box<dyn Strategy>, Box<dyn Error>> {
        let strategy: Box<dyn Strategy> = match name {
            InstitutionName::SoFiBank => Box::new(SoFiBankStrategy),
            InstitutionName::EverBank => Box::new(EverBankStrategy),
        };

        Ok(strategy)
    }
}
