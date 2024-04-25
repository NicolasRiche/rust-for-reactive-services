use reactive_service_domain::order_state::Invoice;

#[derive(Debug, Clone)]
pub struct PaymentToken(String);

pub trait PaymentProcessor {
    fn pay_with_token(&self, payment_token: PaymentToken) -> Invoice;
}

pub struct LocalPaymentProcessor {}

impl PaymentProcessor for LocalPaymentProcessor {
    fn pay_with_token(&self, payment_token: PaymentToken) -> Invoice {
        let _ = payment_token.0;
        Invoice{}
    }
}