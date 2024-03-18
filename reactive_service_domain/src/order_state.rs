use std::collections::HashMap;

use crate::canada_postal_code::CanadaPostalCode;

pub enum OrderState {
    OrderInitiated(OrderInitiated),
    OrderWithAddress(OrderWithAddress),
    OrderCompleted(OrderCompleted)
}

impl Default for OrderState {
    fn default() -> Self {
        OrderState::OrderInitiated(OrderInitiated::default())
    }
}


pub struct OrderInitiated {
    cart: HashMap<ProductId, Quantity>
}

impl Default for OrderInitiated {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderInitiated {
    pub fn new() -> Self { Self::default() }

    pub fn get_cart(&self) -> &HashMap<ProductId, Quantity> { &self.cart }

    pub fn with_cart(self, cart: HashMap<ProductId, Quantity>) -> Self {
        Self { cart }
    }

    pub fn with_delivery_address(self, delivery_address: DeliveryAddress, shipping_cost: Money, tax: Money) -> OrderWithAddress {
        OrderWithAddress {
            cart: self.cart,
            delivery_address,
            shipping_cost,
            tax,
        }
    }
    
}

pub struct OrderWithAddress {
    cart: HashMap<ProductId, Quantity>,
    delivery_address: DeliveryAddress,
    shipping_cost: Money,
    tax: Money
}

impl OrderWithAddress {

    pub fn with_cart(self, cart: HashMap<ProductId, Quantity>, shipping_cost: Money, tax: Money) -> Self {
        Self { cart, shipping_cost, tax, ..self }
    }

    pub fn with_delivery_address(self, delivery_address: DeliveryAddress, shipping_cost: Money, tax: Money) -> OrderWithAddress {
        Self { delivery_address, shipping_cost, tax, ..self }
    }

    pub fn complete_order(self, invoice_id: InvoiceId) -> OrderCompleted {
        OrderCompleted{
            cart: self.cart,
            delivery_address: self.delivery_address,
            shipping_cost: self.shipping_cost,
            tax: self.tax,
            invoice_id
        }
    }
}

pub struct OrderCompleted {
    cart: HashMap<ProductId, Quantity>,
    delivery_address: DeliveryAddress,
    shipping_cost: Money,
    tax: Money,
    invoice_id: InvoiceId
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ProductId(String);

#[derive(Clone, PartialEq, Eq)]
pub struct Quantity(u16);

#[derive(Clone)]
pub struct DeliveryAddress {
    street: Street,
    postal_code: CanadaPostalCode
}

#[derive(Clone)]
struct Street(String);

#[derive(PartialEq, Eq, Hash)]
pub struct InvoiceId(String);

#[derive(Clone)]
pub struct Money {
    pub amount_cents: u32,
    pub currency: Currency
}

#[derive(Clone)]
pub enum Currency {
    Cad
}

