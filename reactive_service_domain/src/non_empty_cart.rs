use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sku(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Quantity(pub u16);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonEmptyCart {
    cart: HashMap<Sku, Quantity>
}

impl NonEmptyCart {
    pub fn new(cart: HashMap<Sku, Quantity>) -> Result<Self, &'static str> {
       if cart.is_empty() { Err("Cart can't be empty") }
       else { Ok(Self{cart}) }
    }
}