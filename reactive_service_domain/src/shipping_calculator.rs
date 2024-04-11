use crate::non_empty_cart::NonEmptyCart;
use crate::order_state::{Currency, DeliveryAddress, Money};

pub trait ShippingCalculator {
    fn shipping_cost(&self, cart: &NonEmptyCart, delivery_address: &DeliveryAddress) -> Money;
}

pub struct LocalShippingCalculator {}

impl ShippingCalculator for LocalShippingCalculator {
    fn shipping_cost(&self, cart: &NonEmptyCart, delivery_address: &DeliveryAddress) -> Money {
        let _ = cart;
        let _ = delivery_address;
        Money {
            amount_cents: 200,
            currency: Currency::Cad
        }
    }
}