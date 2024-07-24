use core::fmt;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub enum InstitutionName {
    SoFiBank,
    EverBank,
}

impl InstitutionName {
    fn as_str(&self) -> &str {
        match *self {
            InstitutionName::SoFiBank => "SoFi Bank",
            InstitutionName::EverBank => "EverBank",
        }
    }
}

impl FromStr for InstitutionName {
    type Err = ParseInstitutionError;

    fn from_str(s: &str) -> Result<InstitutionName, ParseInstitutionError> {
        match s {
            "SoFi Bank" => Ok(InstitutionName::SoFiBank),
            "EverBank" => Ok(InstitutionName::EverBank),
            _ => Err(ParseInstitutionError),
        }
    }
}

#[derive(Debug)]
pub struct ParseInstitutionError;

impl fmt::Display for ParseInstitutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid institution name")
    }
}

impl Error for ParseInstitutionError {}

impl fmt::Display for InstitutionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
