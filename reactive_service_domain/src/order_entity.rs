use std::collections::HashMap;

type SeqEvent = SequencedEvent<OrderEvent>;
type Events = Vec<SeqEvent>;

use crate::aggregate_root::{
    AggregateRoot, AggregateCommandResult, SequencedEvent
};
use crate::non_empty_cart::NonEmptyCart;
use crate::order_state::{
    Currency, DeliveryAddress, Empty, Money, OrderState, WithAddress, WithCart
};

struct OrderEntity{
    order_state: OrderState,
    sequence_number: u64
}

impl AggregateRoot for OrderEntity {
    type State = OrderState;
    type Command = OrderCommand;
    type Error = &'static str;
    type Event = OrderEvent;
    
    fn load_from_events(&mut self, events: Vec<SequencedEvent<Self::Event>>) -> &Self::State {
       for event in events {
         let current_state = std::mem::take(&mut self.order_state);
         self.order_state = Self::apply_event(current_state, event);
       }
       &self.order_state
    }

    fn handle_command(&mut self, command: Self::Command) -> Result<AggregateCommandResult<Self::State, Self::Event>, Self::Error> {
        // Handle required mutations here
        // Passing only immutables data to a pure function that return new state and the events
        let current_state = std::mem::take(&mut self.order_state);
        match Self::handle_command_with_state(self.sequence_number, current_state, command) {
           Ok((new_state, events)) => {
                self.order_state = new_state;
                if let Some(last_event) = events.last() {
                    self.sequence_number = last_event.sequence_number
                }
                Ok(AggregateCommandResult{state: &self.order_state, events})
            },
            Err((current_state, err)) => {
                self.order_state = current_state;
                Err(err)
            }
        }
    }
    
}

impl OrderEntity {

    fn handle_command_with_state(current_seq_number: u64, current_state: OrderState, command: OrderCommand) -> Result<(OrderState, Events), (OrderState, &'static str)> {
        match current_state {
            OrderState::Empty(order_empty) => 
              Self::empty_command_handler(current_seq_number, order_empty, command),
            OrderState::WithCart(order_with_cart) => 
                Self::with_cart_command_handler(current_seq_number, order_with_cart, command),
            OrderState::WithAddress(_) => todo!(),
            OrderState::Completed(_) => todo!(),
        }
    }

    fn empty_command_handler(current_seq_number: u64, order_empty: Empty, command: OrderCommand) ->
        Result<(OrderState, Events), (OrderState, &'static str)> {

        match command {
            OrderCommand::UpdateCart { cart } => {
                let new_state = OrderState::WithCart(order_empty.with_cart(cart.clone()));
                let seq_number = current_seq_number + 1;
                let events = vec![
                    SequencedEvent{ sequence_number: seq_number, event: OrderEvent::UpdatedCart{cart}}
                    ];
                Ok((new_state,events))
            },
            OrderCommand::AddOrUpdateDeliveryAddress{..} => 
                Err((OrderState::Empty(order_empty), "Can't add a delivery address on an empty cart")),
            OrderCommand::Pay{..} => 
                Err((OrderState::Empty(order_empty), "Order is not ready for payment")),
        }
    }


    fn with_cart_command_handler(current_seq_number: u64, order_with_cart: WithCart, command: OrderCommand) ->
        Result<(OrderState, Events), (OrderState, &'static str)> {

        match command {
            OrderCommand::UpdateCart { cart } => {
                let new_state = OrderState::WithCart(order_with_cart.with_cart(cart.clone()));
                let seq_number = current_seq_number + 1;
                let events = vec![
                    SequencedEvent{ sequence_number: seq_number, event: OrderEvent::UpdatedCart{cart}}
                    ];
                Ok((new_state,events))
            },
            OrderCommand::AddOrUpdateDeliveryAddress { delivery_address } => {
                let shipping_cost = Money{amount_cents: 4000u32, currency: Currency::Cad };
                let tax = Money{amount_cents: 1200u32, currency: Currency::Cad };
                let new_state = OrderState::WithAddress(order_with_cart.with_delivery_address(delivery_address.clone(), shipping_cost.clone(), tax.clone()));
                let seq_number = current_seq_number + 1;
                let events = vec![
                    SequencedEvent{ 
                        sequence_number: seq_number, 
                        event: OrderEvent::AddedOrUpdateDeliveryAddress{
                            delivery_address,
                            shipping_cost,
                            tax
                        }
                    }
                    ];
                Ok((new_state,events))
            },
            OrderCommand::Pay{..} => Err((OrderState::WithCart(order_with_cart), "Order is not ready for payment")),
        }
    }

    fn with_addr_command_handler(current_seq_number: u64, order_with_addr: WithAddress, command: OrderCommand) -> 
        Result<(OrderState, Events), (OrderState, &'static str)> {

        match command {
            OrderCommand::UpdateCart { cart } => {
                let shipping_cost = Money{amount_cents: 4000u32, currency: Currency::Cad };
                let tax = Money{amount_cents: 1200u32, currency: Currency::Cad };

                let new_state = OrderState::WithAddress(order_with_addr.with_cart(cart.clone(), shipping_cost, tax));
                let seq_number = current_seq_number + 1;
                let events = vec![
                    SequencedEvent{ sequence_number: seq_number, event: OrderEvent::UpdatedCart{cart}}
                ];
                Ok((new_state, events))
            },
            OrderCommand::AddOrUpdateDeliveryAddress { delivery_address } => {
                let shipping_cost = Money{amount_cents: 4000u32, currency: Currency::Cad };
                let tax = Money{amount_cents: 1200u32, currency: Currency::Cad };

                let new_state = OrderState::WithAddress(order_with_addr.with_delivery_address(
                    delivery_address.clone(), shipping_cost.clone(), tax.clone())
                );

                let seq_number = current_seq_number + 1;
                let events = vec![
                    SequencedEvent{ 
                        sequence_number: seq_number, 
                        event: OrderEvent::AddedOrUpdateDeliveryAddress{
                            delivery_address, shipping_cost, tax
                        }
                    }
                ];
                Ok((new_state, events))
            },
            OrderCommand::Pay{..} => Err((OrderState::WithAddress(order_with_addr), "Order is not ready for payment")),
        }
    }

    fn apply_event(order_state: OrderState, seq_evt: SequencedEvent<OrderEvent>) -> OrderState {
        match order_state {
            OrderState::Empty(_) => todo!(),
            OrderState::WithCart(order_initiated) => {
                match seq_evt.event {
                    OrderEvent::UpdatedCart { cart } =>
                        OrderState::WithCart(order_initiated.with_cart(cart)),
                    OrderEvent::AddedOrUpdateDeliveryAddress { delivery_address, shipping_cost, tax } =>
                       OrderState::WithAddress(order_initiated.with_delivery_address(delivery_address, shipping_cost, tax)),
                    OrderEvent::Paid{..} =>
                        panic!("Cannot apply Paid event to an InitiatedOrder")
                }
            }
            OrderState::WithAddress(_) => todo!(),
            OrderState::Completed(_) => todo!(),
        }
    }

}

#[derive(Clone)]
enum OrderEvent{
    UpdatedCart{cart: NonEmptyCart},
    AddedOrUpdateDeliveryAddress{
        delivery_address: DeliveryAddress,
        shipping_cost: Money,
        tax: Money
    },
    Paid{payment_token: PaymentToken}
}

enum OrderCommand {
    UpdateCart{cart: NonEmptyCart},
    AddOrUpdateDeliveryAddress{delivery_address: DeliveryAddress},
    Pay{payment_token: PaymentToken}

}

#[derive(Clone)]
struct PaymentToken(String);
