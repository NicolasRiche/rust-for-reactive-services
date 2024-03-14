extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

use std::str::FromStr;
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
/// assert_eq!(format!("{:?}", PostalCode::try_from("A1A 1A1").unwrap()), "PostalCode(\"A1A 1A1\")");
/// assert_eq!(format!("{:?}", PostalCode::try_from("A1A1A1").unwrap()), "PostalCode(\"A1A 1A1\")");
/// ```
#[derive(Clone, Debug)]
pub struct PostalCode(heapless::String<7>);

impl TryFrom<&str> for PostalCode {
    type Error = PostalCodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let trim_input = value.trim();

        if POSTAL_CODE_WITH_SPACE.is_match(trim_input) {
            Ok(Self(heapless::String::from_str(trim_input).unwrap()))

        } else if POSTAL_CODE_NO_SPACE.is_match(trim_input) {
            let with_space = {
                let (first_part, second_part) = trim_input.split_at(3);
                format!("{} {}", first_part, second_part)
            };
            Ok(Self(heapless::String::from_str(with_space.as_str()).unwrap()))
            
        } else {
            Err(PostalCodeError::InvalidPostalCode)
        }
    }
}

lazy_static! {
    static ref POSTAL_CODE_WITH_SPACE: Regex = Regex::new(r"^[A-Z]\d[A-Z] \d[A-Z]\d$").unwrap();
    static ref POSTAL_CODE_NO_SPACE: Regex = Regex::new(r"^[A-Z]\d[A-Z]\d[A-Z]\d$").unwrap();
}
