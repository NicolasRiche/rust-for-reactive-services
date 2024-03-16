pub mod canada_postal_code;
pub mod canada_postal_code_nutype;

use canada_postal_code::CanadaPostalCode;

fn main() {
    let _ = test();
    ()
}

fn test() -> Result<(),&'static str> {
    match CanadaPostalCode::try_from("ABCDEF") {
        Ok(postal_code) => println!("{}", postal_code),
        Err(str) => println!("Error when parsing the Postal code {}", str)
    }
    Ok(())
}
