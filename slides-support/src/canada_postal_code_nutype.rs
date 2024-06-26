use nutype::nutype;

#[nutype(
    sanitize(trim, uppercase, with = |s| s.replace(' ', "")),
    validate(regex = r"^[ABCEGHJKLMNPRSTVXY]\d[ABCEGHJKLMNPRSTVWXYZ]\ ?\d[ABCEGHJKLMNPRSTVWXYZ]\d$"),
    derive(Debug, Display, PartialEq, FromStr),
)]
pub struct CanadaPostalCodeNuType(String);
