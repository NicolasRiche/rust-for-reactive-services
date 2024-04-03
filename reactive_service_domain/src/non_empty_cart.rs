use std::collections::HashMap;

use crate::order_state::Money;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sku(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quantity(u16);

#[derive(Debug, Clone)]
pub struct NonEmptyCart {
    cart: HashMap<Sku, Quantity>
}

impl NonEmptyCart {
    pub fn new(cart: HashMap<Sku, Quantity>) -> Result<Self, &'static str> {
       if cart.is_empty() { Err("Cart can't be empty") }
       else { Ok(Self{cart}) }
    }
}