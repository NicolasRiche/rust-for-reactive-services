pub mod canada_postal_code;
pub mod canada_postal_code_nutype;

use canada_postal_code::CanadaPostalCode;
use canada_postal_code_nutype::CanadaPostalCodeNuType;

fn main() {
    let postal_code = CanadaPostalCode::try_from("K1B 0A1").unwrap();
    println!("{}", postal_code);

    let postal_code = CanadaPostalCode::try_from("K1B0A1").unwrap();
    println!("{}", postal_code);

    let postal_code: CanadaPostalCodeNuType = CanadaPostalCodeNuType::try_from("K1B 001").unwrap();
    println!("{}", postal_code);


}
