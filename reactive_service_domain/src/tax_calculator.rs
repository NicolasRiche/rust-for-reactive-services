use crate::non_empty_cart::NonEmptyCart;
use crate::order_state::{Currency, DeliveryAddress, Money};

pub struct TaxCalculator {}

impl TaxCalculator {
    pub fn tax_cost(&self, cart: &NonEmptyCart, shipping_cost: &Money) -> Money {
        let _ = cart;
        let _ = shipping_cost;
        Money {
            amount_cents: 130,
            currency: Currency::Cad
        }
    }
}