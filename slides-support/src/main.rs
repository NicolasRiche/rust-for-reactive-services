pub mod canada_postal_code;

use canada_postal_code::CanadaPostalCode;

fn main() {
    let postal_code = CanadaPostalCode::try_from("K1B 0A1").unwrap();
    println!("{}", postal_code);

    let postal_code = CanadaPostalCode::try_from("K1B0A1").unwrap();
    println!("{}", postal_code);

}
