use reactive_service_domain::non_empty_cart::NonEmptyCart;
use reactive_service_domain::order_state::{Currency, DeliveryAddress, Money};
use crate::order_service::ShippingCalculator;

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