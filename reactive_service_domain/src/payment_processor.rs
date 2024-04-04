use crate::{order_entity::PaymentToken, order_state::Invoice};


pub struct PaymentProcessor {}

impl PaymentProcessor {
    pub fn pay_with_token(&self, payment_token: PaymentToken) -> Invoice {
        let _ = payment_token;
        Invoice{}
    }
}