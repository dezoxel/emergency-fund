use std::error::Error;

use crate::institution::InstitutionName;
use super::strategy::SoFiBankStrategy;
use super::strategy::EverBankStrategy;
use super::Strategy;
use super::StrategyFactory;

impl StrategyFactory {
    pub fn create(&self, name: &InstitutionName) -> Result<Box<dyn Strategy>, Box<dyn Error>> {
        let strategy: Box<dyn Strategy> = match name {
            InstitutionName::SoFiBank => Box::new(SoFiBankStrategy),
            InstitutionName::EverBank => Box::new(EverBankStrategy),
        };

        Ok(strategy)
    }
}
