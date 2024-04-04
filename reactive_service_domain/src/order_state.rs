use crate::{canada_postal_code::CanadaPostalCode, non_empty_cart::NonEmptyCart};


///
// stateDiagram-v2
//     [*] --> Empty
//     Empty --> WithCart: add_cart(NonEmptyCart)
// 
//     WithCart --> WithCart: update_cart(NonEmptyCart)
// 
//     WithCart --> WithAddress: add_delivery_address(DeliveryAddress, ShippingCost, Tax)
//     
//     WithAddress --> WithAddress: update_cart(NonEmptyCart, ShippingCost, Tax)<br/> or <br/>update_delivery_address(DeliveryAddress, ShippingCost, Tax)
// 
//     WithAddress --> Completed: pay_with_token(PaymentToken)
#[derive(Debug)]
pub enum OrderState {
    Empty(Empty),
    WithCart(WithCart),
    WithAddress(WithAddress),
    Completed(Completed)
}

impl Default for OrderState {
    fn default() -> Self {
        OrderState::Empty(Empty{})
    }
}

#[derive(Debug)]
pub struct Empty{} 
impl Empty {
    pub fn add_cart(self, cart: NonEmptyCart) -> WithCart { 
        WithCart { cart } 
    }
}

#[derive(Debug)]
pub struct WithCart {
    cart: NonEmptyCart
}

impl WithCart {
    pub fn get_cart(&self) -> &NonEmptyCart { &self.cart }

    pub fn update_cart(self, cart: NonEmptyCart) -> Self { 
        Self { cart } 
    }

    pub fn add_delivery_address(self, delivery_address: DeliveryAddress, shipping_cost: Money, tax: Money) -> WithAddress {
        WithAddress {
            cart: self.cart,
            delivery_address,
            shipping_cost,
            tax,
        }
    }
    
}

#[derive(Debug)]
pub struct WithAddress {
    cart: NonEmptyCart,
    delivery_address: DeliveryAddress,
    shipping_cost: Money,
    tax: Money
}

impl WithAddress {
    pub fn get_cart(&self) -> &NonEmptyCart { &self.cart }
    pub fn get_delivery_address(&self) -> &DeliveryAddress { &self.delivery_address }

    pub fn update_cart(self, cart: NonEmptyCart, shipping_cost: Money, tax: Money) -> Self {
        Self { cart, shipping_cost, tax, ..self }
    }

    pub fn update_delivery_address(self, delivery_address: DeliveryAddress, shipping_cost: Money, tax: Money) -> Self {
        Self { delivery_address, shipping_cost, tax, ..self }
    }

    pub fn complete_order(self, invoice: Invoice) -> Completed {
        Completed {
            cart: self.cart,
            delivery_address: self.delivery_address,
            shipping_cost: self.shipping_cost,
            tax: self.tax,
            invoice
        }
    }
}

#[derive(Debug)]
pub struct Completed {
    cart: NonEmptyCart,
    delivery_address: DeliveryAddress,
    shipping_cost: Money,
    tax: Money,
    invoice: Invoice
}

#[derive(Debug, Clone)]
pub struct DeliveryAddress {
    street: Street,
    postal_code: CanadaPostalCode
}

#[derive(Debug, Clone)]
struct Street(String);

#[derive(Debug, Clone)]
pub struct Money {
    pub amount_cents: u32,
    pub currency: Currency
}

#[derive(Debug, Clone)]
pub enum Currency {
    Cad
}

#[derive(Debug, Clone)]
pub struct Invoice{}

