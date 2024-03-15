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

/// Canadian Postal code. Validate input with or without the middle optional space (A1A 1A1 or A1A1A1), letters should be uppercase.
/// If the input pass validation, the postal code is stored with space: A1A 1A1
///
/// # Examples
///
/// ```
/// # use std::convert::TryFrom;
/// # use reactive_service_domain::postal_code::{PostalCode, PostalCodeError};
/// assert_eq!(PostalCode::try_from("A1A 1A1").is_ok(), true);
/// assert_eq!(PostalCode::try_from("A1A1A1").is_ok(), true);
///
/// assert_eq!(PostalCode::try_from("a1a 1a1").is_err(), true);
/// assert_eq!(PostalCode::try_from("A1A").is_err(), true);
/// assert_eq!(PostalCode::try_from("A1A 1A1 A1A").is_err(), true);
/// assert_eq!(PostalCode::try_from("21111").is_err(), true);
/// assert_eq!(format!("{}", PostalCode::try_from("A1A 1A1").unwrap()), "A1A 1A1");
/// assert_eq!(format!("{}", PostalCode::try_from("A1A1A1").unwrap()), "A1A 1A1");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostalCode(heapless::String<6>);

impl TryFrom<&str> for PostalCode {
    type Error = PostalCodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let trim_input = value.trim();

        if CANADA_POSTAL_CODE.is_match(trim_input) {
            let no_space = trim_input.replace(' ', "");
            Ok(Self(heapless::String::from_str(&no_space).unwrap()))
        } else {
            Err(PostalCodeError::InvalidPostalCode)
        }
    }
}

impl Display for PostalCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (first_part, second_part) = self.0.split_at(3);
        write!(f, "{} {}", first_part, second_part)
    }
}

lazy_static! {
    static ref CANADA_POSTAL_CODE: Regex = 
      Regex::new(r"^[ABCEGHJKLMNPRSTVXY]\d[ABCEGHJKLMNPRSTVWXYZ]\ ?\d[ABCEGHJKLMNPRSTVWXYZ]\d$").unwrap();
}
