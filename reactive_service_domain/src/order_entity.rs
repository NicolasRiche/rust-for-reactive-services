use std::collections::HashMap;

type SeqEvent = SequencedEvent<OrderEvent>;
type Events = Vec<SeqEvent>;

use crate::aggregate_root::{
    AggregateRoot, SequencedEvent
};
use crate::order_state::{
    Currency, DeliveryAddress, Money, OrderInitiated, OrderState, OrderWithAddress, ProductId, Quantity
};


struct OrderId(String);
struct OrderEntity{
    id: OrderId,
    order_state: OrderState,
    sequence_number: u64
}

impl AggregateRoot for OrderEntity {
    type EntityId = OrderId;
    type Command = OrderCommand;
    type Event = OrderEvent;
    type State = OrderState;
    type Error = &'static str;

    fn handle_command(&mut self, command: Self::Command) -> Result<(&Self::State, Vec<SequencedEvent<Self::Event>>), Self::Error> {
       let (new_state, events) = match &self.order_state {
            OrderState::OrderInitiated(order_initiated) => 
                OrderEntity::initiated_command_handler(self.sequence_number, order_initiated, command)?,
            OrderState::OrderWithAddress(order_with_addr) => 
                OrderEntity::with_addr_command_handler(self.sequence_number, order_with_addr, command)?,
            OrderState::OrderCompleted(_) => todo!(),
        };

        self.order_state = new_state;
        if let Some(last_event) = events.last() {
            self.sequence_number = last_event.sequence_number
        }
        Ok((&self.order_state, events))
    }

    fn apply_event(&mut self, seq_evt: SequencedEvent<Self::Event>) -> &Self::State {
        match &mut self.order_state {
            OrderState::OrderInitiated(order_initiated) => {
                match seq_evt.event {
                    OrderEvent::UpdatedCart { cart } => {
                        let new_state = OrderState::OrderInitiated(order_initiated.with_cart(cart));
                        self.order_state = new_state;
                        &self.order_state
                    },
                    OrderEvent::AddedOrUpdateDeliveryAddress { delivery_address, shipping_cost, tax } => {
                        let new_state = OrderState::OrderWithAddress(order_initiated.with_delivery_address(delivery_address, shipping_cost, tax));
                        self.order_state = new_state;
                        &self.order_state
                    }
                    OrderEvent::Paid { payment_token } => 
                    panic!("Cannot apply Paid event to an InitiatedOrder")
                    ,
                }
            }
            OrderState::OrderWithAddress(_) => todo!(),
            OrderState::OrderCompleted(_) => todo!(),
        }
    }

    
}

impl OrderEntity {

    fn initiated_command_handler(current_seq_number: u64, order_initiated: &OrderInitiated, command: OrderCommand) -> 
        Result<(OrderState, Events), &'static str> {

        match command {
            OrderCommand::UpdateCart { cart } => {
                let new_state = OrderState::OrderInitiated(order_initiated.with_cart(cart.clone()));
                let seq_number = current_seq_number + 1;
                let events = vec![
                    SequencedEvent{ sequence_number: seq_number, event: OrderEvent::UpdatedCart{cart}}
                    ];
                Ok((new_state, events))
            },
            OrderCommand::AddOrUpdateDeliveryAddress { delivery_address } => {
                let shipping_cost = Money{amount_cents: 4000u32, currency: Currency::Cad };
                let tax = Money{amount_cents: 1200u32, currency: Currency::Cad };
                let new_state = OrderState::OrderWithAddress(order_initiated.with_delivery_address(delivery_address.clone(), shipping_cost.clone(), tax.clone()));
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
                Ok((new_state, events))
            },
            OrderCommand::Pay{..} => Err("Order is not ready for payment"),
        }
    }

    fn with_addr_command_handler(current_seq_number: u64, order_with_addr: &OrderWithAddress, command: OrderCommand) -> 
        Result<(OrderState, Events), &'static str> {

        match command {
            OrderCommand::UpdateCart { cart } => {
                let shipping_cost = Money{amount_cents: 4000u32, currency: Currency::Cad };
                let tax = Money{amount_cents: 1200u32, currency: Currency::Cad };

                let new_state = OrderState::OrderWithAddress(order_with_addr.with_cart(cart.clone(), shipping_cost, tax));
                let seq_number = current_seq_number + 1;
                let events = vec![
                    SequencedEvent{ sequence_number: seq_number, event: OrderEvent::UpdatedCart{cart}}
                    ];
                Ok((new_state, events))
            },
            OrderCommand::AddOrUpdateDeliveryAddress { delivery_address } => {
                let shipping_cost = Money{amount_cents: 4000u32, currency: Currency::Cad };
                let tax = Money{amount_cents: 1200u32, currency: Currency::Cad };

                let new_state = OrderState::OrderWithAddress(order_with_addr.with_delivery_address(delivery_address.clone(), shipping_cost.clone(), tax.clone()));

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
                Ok((new_state, events))
            },
            OrderCommand::Pay{..} => Err("Order is not ready for payment"),
        }
    }

}

#[derive(Clone)]
enum OrderEvent{
    UpdatedCart{cart: HashMap<ProductId, Quantity>},
    AddedOrUpdateDeliveryAddress{
        delivery_address: DeliveryAddress,
        shipping_cost: Money,
        tax: Money
    },
    Paid{payment_token: PaymentToken}
}

enum OrderCommand {
    UpdateCart{cart: HashMap<ProductId, Quantity>},
    AddOrUpdateDeliveryAddress{delivery_address: DeliveryAddress},
    Pay{payment_token: PaymentToken}

}

#[derive(Clone)]
struct PaymentToken(String);
