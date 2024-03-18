extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostalCodeError {
    #[error("Invalid postal code format")]
    InvalidPostalCode,
}

/// Canadian Postal code. Validate input with or without the middle optional space (A1A 0B0 or A1A0B0).
/// Input is sanitized with trim and uppercase.
/// If the input pass validation, the postal code is stored as 6 chars without space: A1A0B0.
/// Display implementation prints the postal code with the middle space: A1A 0B0
///
/// # Examples
///
/// ```
/// # use std::convert::TryFrom;
/// # use reactive_service_domain::canada_postal_code::CanadaPostalCode;
/// assert_eq!(CanadaPostalCode::try_from("A1A 0B0").is_ok(), true);
/// assert_eq!(CanadaPostalCode::try_from("A1A0B0").is_ok(), true);
///
/// assert_eq!(CanadaPostalCode::try_from("a1a 0b0").is_ok(), true);
/// 
/// assert_eq!(CanadaPostalCode::try_from("A1A").is_err(), true);
/// assert_eq!(CanadaPostalCode::try_from("A1A 0B0 A1A").is_err(), true);
/// assert_eq!(CanadaPostalCode::try_from("21111").is_err(), true);
/// assert_eq!(format!("{}", CanadaPostalCode::try_from("A1A 0B0").unwrap()), "A1A 0B0");
/// assert_eq!(format!("{}", CanadaPostalCode::try_from("A1A0B0").unwrap()), "A1A 0B0");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanadaPostalCode(heapless::String<6>);

impl TryFrom<&str> for CanadaPostalCode {
    type Error = PostalCodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let sanitized = value.trim().to_uppercase();

        if CANADA_POSTAL_CODE.is_match(&sanitized) {
            let no_space = sanitized.replace(' ', "");
            Ok(Self(heapless::String::from_str(&no_space).unwrap()))
        } else {
            Err(PostalCodeError::InvalidPostalCode)
        }
    }
}

impl Display for CanadaPostalCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (first_part, second_part) = self.0.split_at(3);
        write!(f, "{} {}", first_part, second_part)
    }
}

lazy_static! {
    static ref CANADA_POSTAL_CODE: Regex = 
      Regex::new(r"^[ABCEGHJKLMNPRSTVXY]\d[ABCEGHJKLMNPRSTVWXYZ]\ ?\d[ABCEGHJKLMNPRSTVWXYZ]\d$").unwrap();
}