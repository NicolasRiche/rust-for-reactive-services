use crate::aggregate_root::{AggregateRoot, SequencedEvent};
use crate::non_empty_cart::NonEmptyCart;
use crate::order_state::{Completed, DeliveryAddress, Empty, Invoice, Money, OrderState, WithAddress, WithCart};
use crate::payment_processor::PaymentProcessor;
use crate::shipping_calculator::ShippingCalculator;
use crate::tax_calculator::TaxCalculator;

pub struct OrderEntity{
    order_state: OrderState,
    sequence_number: u64,
    shipping_calculator: ShippingCalculator,
    tax_calculator: TaxCalculator,
    payment_processor: PaymentProcessor
}

impl Default for OrderEntity {
    fn default() -> Self {
        OrderEntity{
            order_state: OrderState::Empty(Empty{}),
            sequence_number: 0,
            shipping_calculator: ShippingCalculator{},
            tax_calculator: TaxCalculator{},
            payment_processor: PaymentProcessor{}
        }
    }
}

impl AggregateRoot for OrderEntity {
    type State = OrderState;
    type Command = OrderCommand;
    type Error = &'static str;
    type Event = OrderEvent;
    
    fn restore_from_events(&mut self, events: Vec<SequencedEvent<Self::Event>>) -> Result<&Self::State, Self::Error> {
        for seq_event in events {
            let current_state = std::mem::replace(&mut self.order_state, OrderState::Empty(Empty{}));
            match Self::apply_event(current_state, seq_event.event) {
                Ok(newState) => {
                    self.order_state = newState;
                    self.sequence_number = seq_event.sequence_number
                },
                Err((current_state, err)) => {
                    self.order_state = current_state;
                    return Err(err)
                },
            }
        }
        Ok(&self.order_state)
    }


    fn handle_command(&mut self, command: Self::Command) -> Result<(&Self::State, Vec<SequencedEvent<Self::Event>>), Self::Error> {
        // Handle required mutations here
        // Passing only immutables data to a pure function that return new state and the events

        // Take ownership of the current order state, temporary replace the enum value by default (Empty)
        // which is cheap performance wise
        let current_state = std::mem::take(&mut self.order_state);
        match self.handle_command_with_state(current_state, command) {
            // If success, we have a new state to apply and a sequence of events
            Ok((new_state, events)) => {
                self.order_state = new_state;

                let seq_events = events.iter().map(|evt| {
                    self.sequence_number += 1;
                    SequencedEvent{sequence_number: self.sequence_number, event: evt.to_owned()}
                }).collect();

                Ok((&self.order_state,seq_events))
            },
            // If error the command handler return back the order state, we re-apply it
            // to restore the enum value
            Err((current_state, err)) => {
                self.order_state = current_state;
                Err(err)
            }
        }
    }
    
}

impl OrderEntity {

    fn handle_command_with_state(&self, current_state: OrderState, command: OrderCommand)
        -> Result<(OrderState, Vec<OrderEvent>), (OrderState, &'static str)> {

        match current_state {
            OrderState::Empty(order_empty) =>
                self.empty_order_command_handler(order_empty, command),
            OrderState::WithCart(order_with_cart) =>
                self.with_cart_command_handler(order_with_cart, command),
            OrderState::WithAddress(order_with_addr) =>
                self.with_addr_command_handler(order_with_addr, command),
            OrderState::Completed(completed_order) =>
                self.with_completed_order(completed_order, command)
        }
    }

    fn empty_order_command_handler(&self, order_empty: Empty, command: OrderCommand)
        -> Result<(OrderState, Vec<OrderEvent>), (OrderState, &'static str)> {

        match command {
            OrderCommand::UpdateCart { cart } => {
                let new_state = OrderState::WithCart(order_empty.add_cart(cart.clone()));
                let events = vec![OrderEvent::UpdatedCart{cart}];
                Ok((new_state,events))
            },
            OrderCommand::UpdateDeliveryAddress{..} => 
                Err((OrderState::Empty(order_empty), "Can't add a delivery address on an empty cart")),
            OrderCommand::Pay{..} => 
                Err((OrderState::Empty(order_empty), "Order is not ready for payment")),
        }
    }


    fn with_cart_command_handler(&self, order_with_cart: WithCart, command: OrderCommand)
        -> Result<(OrderState, Vec<OrderEvent>), (OrderState, &'static str)> {

        match command {
            OrderCommand::UpdateCart { cart } => {
                let new_state = OrderState::WithCart(order_with_cart.update_cart(cart.clone()));
                let events = vec![OrderEvent::UpdatedCart{cart}];
                Ok((new_state,events))
            },
            OrderCommand::UpdateDeliveryAddress { delivery_address } => {
                let shipping_cost =
                    self.shipping_calculator.shipping_cost(order_with_cart.get_cart(), &delivery_address);
                let tax =
                    self.tax_calculator.tax_cost(order_with_cart.get_cart(), &shipping_cost);

                let new_state = OrderState::WithAddress(order_with_cart.add_delivery_address(delivery_address.clone(), shipping_cost.clone(), tax.clone()));
                let events = vec![
                    OrderEvent::UpdatedDeliveryAddress { delivery_address, shipping_cost, tax }
                ];
                Ok((new_state,events))
            },
            OrderCommand::Pay{..} => Err((OrderState::WithCart(order_with_cart), "Order is not ready for payment")),
        }
    }

    fn with_addr_command_handler(&self, order_with_addr: WithAddress, command: OrderCommand)
        -> Result<(OrderState, Vec<OrderEvent>), (OrderState, &'static str)> {

        match command {
            OrderCommand::UpdateCart { cart } => {
                let shipping_cost =
                    self.shipping_calculator.shipping_cost(&cart, order_with_addr.get_delivery_address());
                let tax =
                    self.tax_calculator.tax_cost(&cart, &shipping_cost);

                let new_state = OrderState::WithAddress(order_with_addr.update_cart(cart.clone(), shipping_cost.clone(), tax.clone()));
                let events = vec![OrderEvent::UpdatedCartOnExistingDeliveryAddress {cart, shipping_cost, tax}];
                Ok((new_state, events))
            },
            OrderCommand::UpdateDeliveryAddress { delivery_address } => {
                let shipping_cost =
                    self.shipping_calculator.shipping_cost(order_with_addr.get_cart(), &delivery_address);
                let tax =
                    self.tax_calculator.tax_cost(order_with_addr.get_cart(), &shipping_cost);

                let new_state = OrderState::WithAddress(order_with_addr.update_delivery_address(
                    delivery_address.clone(), shipping_cost.clone(), tax.clone())
                );

                let events = vec![
                    OrderEvent::UpdatedDeliveryAddress { delivery_address, shipping_cost, tax }
                ];
                Ok((new_state, events))
            },
            OrderCommand::Pay{payment_token} => {
                let invoice = self.payment_processor.pay_with_token(payment_token); // use the token
                let new_state = OrderState::Completed(order_with_addr.complete_order(invoice.clone()));
                let events = vec![
                    OrderEvent::Completed { invoice }
                ];
                Ok((new_state, events))
            }
        }
    }

    fn with_completed_order(&self, completed_order: Completed, _command: OrderCommand)
        -> Result<(OrderState, Vec<OrderEvent>), (OrderState, &'static str)> {

        Err((OrderState::Completed(completed_order), "Order is completed"))
    }

    fn apply_event(order_state: OrderState, order_event: OrderEvent)
        -> Result<OrderState, (OrderState, &'static str)> {

        match order_state {
            OrderState::Empty(empty_order) =>
                match order_event {
                    OrderEvent::UpdatedCart { cart } =>
                        Ok(OrderState::WithCart(empty_order.add_cart(cart))),
                    OrderEvent::UpdatedCartOnExistingDeliveryAddress {..} =>
                        Err((OrderState::Empty(empty_order), "Cannot apply UpdatedCart event to an Empty order")),
                    OrderEvent::UpdatedDeliveryAddress {..} =>
                        Err((OrderState::Empty(empty_order), "Cannot apply DeliveryAddress event to an EmptyOrder")),
                    OrderEvent::Completed{..} =>
                        Err((OrderState::Empty(empty_order), "Cannot apply Completed event to an EmptyOrder")),
                }
            ,
            OrderState::WithCart(with_cart) => {
                match order_event {
                    OrderEvent::UpdatedCart { cart } =>
                        Ok(OrderState::WithCart(with_cart.update_cart(cart))),
                    OrderEvent::UpdatedCartOnExistingDeliveryAddress {..} =>
                        Err((OrderState::WithCart(with_cart), "Cannot apply UpdatedCart event to an WithCart order")),
                    OrderEvent::UpdatedDeliveryAddress { delivery_address, shipping_cost, tax } =>
                        Ok(OrderState::WithAddress(with_cart.add_delivery_address(
                            delivery_address, shipping_cost, tax
                        ))),
                    OrderEvent::Completed{..} =>
                        Err((OrderState::WithCart(with_cart), "Cannot apply Completed event to an WithCart order"))
                }
            }
            OrderState::WithAddress(with_addr) =>
                match order_event {
                    OrderEvent::UpdatedCart {..} =>
                        Err((OrderState::WithAddress(with_addr), "Cannot apply AddedOrUpdatedCart event to an WithAddress order")),
                    OrderEvent::UpdatedCartOnExistingDeliveryAddress { cart, shipping_cost, tax } =>
                        Ok(OrderState::WithAddress(with_addr.update_cart(
                            cart, shipping_cost, tax
                        ))),
                    OrderEvent::UpdatedDeliveryAddress { delivery_address, shipping_cost, tax } =>
                        Ok(OrderState::WithAddress(with_addr.update_delivery_address(
                            delivery_address, shipping_cost, tax
                        ))),
                    OrderEvent::Completed{invoice} =>
                        Ok(OrderState::Completed(with_addr.complete_order(invoice))),
                },
            OrderState::Completed(_) =>
                Err((order_state, "Cannot apply further events to a Completed order")),
        }
    }

}

#[derive(Debug, Clone)]
pub enum OrderEvent{
    UpdatedCart {cart: NonEmptyCart},
    UpdatedDeliveryAddress {
        delivery_address: DeliveryAddress,
        shipping_cost: Money,
        tax: Money
    },
    UpdatedCartOnExistingDeliveryAddress {cart: NonEmptyCart, shipping_cost: Money, tax: Money},
    Completed{invoice: Invoice}
}

#[derive(Debug)]
pub enum OrderCommand {
    UpdateCart{cart: NonEmptyCart},
    UpdateDeliveryAddress{delivery_address: DeliveryAddress},
    Pay{payment_token: PaymentToken}
}

#[derive(Debug, Clone)]
pub struct PaymentToken(String);
