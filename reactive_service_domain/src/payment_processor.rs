use crate::{order_entity::PaymentToken, order_state::Invoice};


pub trait PaymentProcessor {
    fn pay_with_token(&self, payment_token: PaymentToken) -> Invoice;
}

struct LocalPaymentProcessor {}

impl PaymentProcessor for LocalPaymentProcessor {
    fn pay_with_token(&self, payment_token: PaymentToken) -> Invoice {
        let _ = payment_token;
        Invoice{}
    }
}