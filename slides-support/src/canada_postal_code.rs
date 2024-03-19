use std::{convert::TryFrom, fmt::Display};

#[derive(Debug, PartialEq)]
pub struct CanadaPostalCode(FirstLetter, Digit, Letter, Digit, Letter, Digit);

impl Display for CanadaPostalCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{} {}{}{}", self.0, self.1, self.2, self.3, self.4, self.5)
    }
}

impl TryFrom<&str> for CanadaPostalCode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Remove spaces and validate length
        let input = value.replace(' ', "");
        if input.len() != 6 {
            return Err("Invalid length");
        }
        // Convert input string to uppercase letters and digits
        let input = input.to_uppercase();

        let mut chars = input.chars();

        // Parse and validate each character
        let l1 = FirstLetter::try_from(chars.next().ok_or("Missing char")?)?;
        let d1 = Digit::try_from(chars.next().ok_or("Missing char")?)?;
        let l2 = Letter::try_from(chars.next().ok_or("Missing char")?)?;
        let d2 = Digit::try_from(chars.next().ok_or("Missing char")?)?;
        let l3 = Letter::try_from(chars.next().ok_or("Missing char")?)?;
        let d3 = Digit::try_from(chars.next().ok_or("Missing char")?)?;

        // If all validations pass, return a new CanadaPostalCode instance
        Ok(CanadaPostalCode(l1, d1, l2, d2, l3, d3))
    }
}

// Define an enum for the digits allowed in a Canadian postcode.
#[derive(Debug, Clone, PartialEq)]
enum Digit {
  Zero,One,Two,Three,Four,Five,Six,Seven,Eight,Nine
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Digit::Zero => '0',
                Digit::One => '1',
                Digit::Two => '2',
                Digit::Three => '3',
                Digit::Four => '4',
                Digit::Five => '5',
                Digit::Six => '6',
                Digit::Seven => '7',
                Digit::Eight => '8',
                Digit::Nine => '9'
            }
        )
    }
}

impl TryFrom<char> for Digit {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Digit::Zero),
            '1' => Ok(Digit::One),
            '2' => Ok(Digit::Two),
            '3' => Ok(Digit::Three),
            '4' => Ok(Digit::Four),
            '5' => Ok(Digit::Five),
            '6' => Ok(Digit::Six),
            '7' => Ok(Digit::Seven),
            '8' => Ok(Digit::Eight),
            '9' => Ok(Digit::Nine),
            _ => Err("Invalid digit"),
        }
    }
}

// Letters allowed in a Canadian postcode.
// D, F, I, O, Q, and U are not used to avoid confusion
#[derive(Debug, Clone, PartialEq)]
enum Letter {
    A, B, C, E, G, H, J, K, L, M, N, P, R, S, T, V, W, X, Y, Z
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Letter::A => 'A',
                Letter::B => 'B',
                Letter::C => 'C',
                Letter::E => 'D',
                Letter::G => 'G',
                Letter::H => 'H',
                Letter::J => 'J',
                Letter::K => 'K',
                Letter::L => 'L',
                Letter::M => 'M',
                Letter::N => 'N',
                Letter::P => 'P',
                Letter::R => 'R',
                Letter::S => 'S',
                Letter::T => 'T',
                Letter::V => 'V',
                Letter::W => 'W',
                Letter::X => 'X',
                Letter::Y => 'Y',
                Letter::Z => 'Z'
            }
        )
    }
}

impl TryFrom<char> for Letter {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Letter::A),
            'B' => Ok(Letter::B),
            'C' => Ok(Letter::C),
            'E' => Ok(Letter::E),
            'G' => Ok(Letter::G),
            'H' => Ok(Letter::H),
            'J' => Ok(Letter::J),
            'K' => Ok(Letter::K),
            'L' => Ok(Letter::L),
            'M' => Ok(Letter::M),
            'N' => Ok(Letter::N),
            'P' => Ok(Letter::P),
            'R' => Ok(Letter::R),
            'S' => Ok(Letter::S),
            'T' => Ok(Letter::T),
            'V' => Ok(Letter::V),
            'W' => Ok(Letter::W),
            'X' => Ok(Letter::X),
            'Y' => Ok(Letter::Y),
            'Z' => Ok(Letter::Z),
            _ => Err("Invalid letter for a Canadian postal code"),
        }
    }
}


// First letters define the region/province.
// As other letters, D, F, I, O, Q, and U are not used to avoid confusion,
// and also does not make use of the letters W or Z.
#[derive(Debug, Clone, PartialEq)]
enum FirstLetter {
    A, B, C, E, G, H, J, K, L, M, N, P, R, S, T, V, X, Y
}

impl Display for FirstLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                FirstLetter::A => 'A',
                FirstLetter::B => 'B',
                FirstLetter::C => 'C',
                FirstLetter::E => 'D',
                FirstLetter::G => 'G',
                FirstLetter::H => 'H',
                FirstLetter::J => 'J',
                FirstLetter::K => 'K',
                FirstLetter::L => 'L',
                FirstLetter::M => 'M',
                FirstLetter::N => 'N',
                FirstLetter::P => 'P',
                FirstLetter::R => 'R',
                FirstLetter::S => 'S',
                FirstLetter::T => 'T',
                FirstLetter::V => 'V',
                FirstLetter::X => 'X',
                FirstLetter::Y => 'Y'
            }
        )
    }
}

impl TryFrom<char> for FirstLetter {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(FirstLetter::A),
            'B' => Ok(FirstLetter::B),
            'C' => Ok(FirstLetter::C),
            'E' => Ok(FirstLetter::E),
            'G' => Ok(FirstLetter::G),
            'H' => Ok(FirstLetter::H),
            'J' => Ok(FirstLetter::J),
            'K' => Ok(FirstLetter::K),
            'L' => Ok(FirstLetter::L),
            'M' => Ok(FirstLetter::M),
            'N' => Ok(FirstLetter::N),
            'P' => Ok(FirstLetter::P),
            'R' => Ok(FirstLetter::R),
            'S' => Ok(FirstLetter::S),
            'T' => Ok(FirstLetter::T),
            'V' => Ok(FirstLetter::V),
            'X' => Ok(FirstLetter::X),
            'Y' => Ok(FirstLetter::Y),
            _ => Err("Invalid first letter for a Canadian postal code"),
        }
    }
}
