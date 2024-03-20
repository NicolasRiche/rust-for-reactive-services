pub mod canada_postal_code;
pub mod canada_postal_code_nutype;
mod payment;


use canada_postal_code::CanadaPostalCode;
use canada_postal_code_nutype::CanadaPostalCodeNuType;

fn main() {
    let _ = payment::usage();
    let _ = test_nutype();
    let _ = test_enums();
    ()
}

fn test_nutype() -> Result<(),&'static str> {

    let postal_code = "K1B 0A1".parse::<CanadaPostalCode>().unwrap();

    Ok(())
}

fn test_enums() -> Result<(),&'static str> {

    let postal_code = "K1B 0A1".parse::<CanadaPostalCode>().unwrap();
    print!("{}", postal_code);

    Ok(())
}