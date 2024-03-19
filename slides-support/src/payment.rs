

#[derive(Debug)]
struct PaymentToken(String);

struct PaymentError {
    error_message: String,
    payment_token: PaymentToken
}

fn pay_with_token(payment_token: PaymentToken) 
  -> Result<(), PaymentError> {
    // Call a fallible external service to process the payment
    external_service(&payment_token)
        .map_err(|err| PaymentError{ 
            error_message: String::from(err), payment_token})
}


pub fn usage() {

    // Usage
    let token = PaymentToken(String::from("test-token"));

    match pay_with_token(token) {
        Ok(_) => {
            // Succeed, the token was consumed.
            // Any attempt to reuse it will trig a compile error
            // println!("My token {:?}", token)
            // Compile error: "borrow of moved value: `token`"
        },
        Err(PaymentError{ error_message, payment_token }) => {
            // Failed, return an error with a message + send back the consumed token
            // The caller can reuse the token to retry
            println!("error {:?}, token {:?} is available for retry attempt", 
            error_message, payment_token)
        }
    }


}
fn external_service(payment_token: &PaymentToken) -> Result<(), &'static str> {
    Ok(())
}