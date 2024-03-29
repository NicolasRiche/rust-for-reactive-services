use std::collections::HashMap;

use crate::order_state::Money;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Sku(String);

#[derive(Clone, PartialEq, Eq)]
pub struct Quantity(u16);

#[derive(Clone)]
pub struct CartItem{
    price: Money,
    quantity: Quantity
}

#[derive(Clone)]
pub struct NonEmptyCart {
    cart: HashMap<Sku, CartItem>
}

impl NonEmptyCart {
    pub fn new(cart: HashMap<Sku, CartItem>) -> Result<Self, &'static str> {
       if cart.is_empty() { Err("Cart can't be empty") }
       else { Ok(Self{cart}) }
    }
}