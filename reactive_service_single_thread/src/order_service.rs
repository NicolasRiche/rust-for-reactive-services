use std::collections::HashMap;
use std::time::Instant;
use reactive_service_domain::aggregate_root::{AggregateRoot, SequencedEvent};
use reactive_service_domain::non_empty_cart::NonEmptyCart;
use reactive_service_domain::order_entity::{OrderEntity, OrderEntityCommand, OrderEvent};
use reactive_service_domain::order_state::{DeliveryAddress, Money, OrderState};
use crate::payment_processor::{PaymentProcessor, PaymentToken};

pub trait EventsJournal<Event> {
    fn persist_event(&mut self, entity_id: OrderId, evt_w_seq: &SequencedEvent<Event>) -> Result<(), &'static str>;
    fn retrieve_events(&mut self, entity_id: OrderId) -> Result<Vec<SequencedEvent<Event>>, &'static str>;
}

pub trait ShippingCalculator {
    fn shipping_cost(&self, cart: &NonEmptyCart, delivery_address: &DeliveryAddress) -> Money;
}

pub trait TaxCalculator {
    fn tax_cost(&self, cart: &NonEmptyCart, shipping_cost: &Money) -> Money;
}

type OrderId = i64;

pub struct OrderService<
    E: EventsJournal<OrderEvent>,
    S: ShippingCalculator,
    T: TaxCalculator,
    P: PaymentProcessor
> {
    orders: HashMap<OrderId, OrderEntity>,
    events_journal: E,
    shipping_calculator: S,
    tax_calculator: T,
    payment_processor: P
}

impl <E, S, T, P> OrderService<E, S, T, P>
where
    E: EventsJournal<OrderEvent>,
    S: ShippingCalculator,
    T: TaxCalculator,
    P: PaymentProcessor
{

    pub fn new(events_journal: E, shipping_calculator: S, tax_calculator: T, payment_processor: P) -> Self {
        Self {
            orders: HashMap::default(),
            events_journal: events_journal,
            shipping_calculator,
            tax_calculator,
            payment_processor
        }
    }

    fn create_and_process_entity_command<F>(&mut self, entity_id: OrderId, create_command: F)
      -> Result<(&OrderState, Vec<SequencedEvent<OrderEvent>>), &'static str>
    where
        F: FnOnce(&mut OrderEntity, &S, &T, &P) -> Result<OrderEntityCommand, &'static str>,
    {
        let order: &mut OrderEntity = self.orders.entry(entity_id)
            .or_insert({
                let events = self.events_journal.retrieve_events(entity_id)?;
                let mut entity = OrderEntity::default();
                let _ = entity.restore_from_events(events)?;
                entity
            });

        let entity_command: OrderEntityCommand = create_command(
            order, &self.shipping_calculator, &self.tax_calculator, &self.payment_processor
        )?;

        let (state, events) = order.handle_command(entity_command)?;

        for evt in &events {
            self.events_journal.persist_event(entity_id, evt)?;
        }
        Ok((state, events))
    }

    pub fn update_cart(&mut self, cmd: UpdateCart)
        -> Result<(&OrderState, Vec<SequencedEvent<OrderEvent>>), &'static str> {

        let update_cart_command_builder =
            |order_entity: &mut OrderEntity, shipping_calculator: &S, tax_calculator: &T, _: &P| -> Result<OrderEntityCommand, &'static str> {

                let cart = cmd.cart;
                match order_entity.get_state() {

                    OrderState::Empty(_) | OrderState::WithCart(_) =>
                        Ok(OrderEntityCommand::AddCart{cart}),

                    OrderState::WithAddress(with_addr) => {
                        let shipping_cost = shipping_calculator.shipping_cost(&cart, with_addr.get_delivery_address());
                        let tax: Money = tax_calculator.tax_cost(&cart, &shipping_cost);
                        Ok(OrderEntityCommand::UpdateCart {cart, shipping_cost, tax})
                    },

                    OrderState::Completed(_) => Err("Can't update the cart on a completed order."),
                }
            };

        self.create_and_process_entity_command(cmd.order_id, update_cart_command_builder)
    }



    pub fn update_delivery_address(&mut self, cmd: UpdateDeliveryAddress)
       -> Result<(&OrderState, Vec<SequencedEvent<OrderEvent>>), &'static str> {

        let update_addr_command_builder =
            |order_entity: &mut OrderEntity, shipping_calculator: &S, tax_calculator: &T, _: &P|
             -> Result<OrderEntityCommand, &'static str> {

                let delivery_address = cmd.delivery_address;
                match order_entity.get_state() {

                    OrderState::Empty(_) => Err("Can't add address to an empty cart."),

                    OrderState::WithCart(with_cart) => {
                        let cart = with_cart.get_cart();
                        let shipping_cost = shipping_calculator.shipping_cost(cart, &delivery_address);
                        let tax: Money = tax_calculator.tax_cost(cart, &shipping_cost);
                        Ok(OrderEntityCommand::UpdateDeliveryAddress { delivery_address, shipping_cost, tax })
                    },

                    OrderState::WithAddress(with_addr) => {
                        let cart = with_addr.get_cart();
                        let shipping_cost = shipping_calculator.shipping_cost(cart, &delivery_address);
                        let tax: Money = tax_calculator.tax_cost(cart, &shipping_cost);
                        Ok(OrderEntityCommand::UpdateDeliveryAddress { delivery_address, shipping_cost, tax })
                    },

                    OrderState::Completed(_) => Err("Can't update address on a completed order."),
                }
            };

        self.create_and_process_entity_command(cmd.order_id, update_addr_command_builder)
    }


    pub fn pay_order(&mut self, cmd: PayOrder)
       -> Result<(&OrderState, Vec<SequencedEvent<OrderEvent>>), &'static str> {

        let pay_order_command_builder =
            |order_entity: &mut OrderEntity, _: &S, _: &T, payment_processor: &P|
             -> Result<OrderEntityCommand, &'static str> {

                let payment_token = cmd.payment_token;
                match order_entity.get_state() {

                    OrderState::Empty(_) | OrderState::WithCart(_) =>
                        Err("Order not ready to be paid."),

                    OrderState::WithAddress(_) => {
                        let invoice = payment_processor.pay_with_token(payment_token);
                        Ok(OrderEntityCommand::Complete{invoice})
                    },

                    OrderState::Completed(_) => Err("Order is already paid."),
                }
            };

        self.create_and_process_entity_command(cmd.order_id, pay_order_command_builder)
    }

    pub fn get_state(&mut self, entity_id: OrderId) -> Result<&OrderState, &'static str> {
        let order: &mut OrderEntity = self.orders.entry(entity_id)
            .or_insert_with(| | {
                let events = 
                    self.events_journal.retrieve_events(entity_id).unwrap();
                let mut entity = OrderEntity::default();
                let _ = entity.restore_from_events(events).unwrap();
                println!("restored events");
                entity
            });

        let state = order.get_state();
        Ok(state)
    }

}

#[derive(Debug, Clone)]
pub struct UpdateCart{pub order_id: OrderId, pub cart: NonEmptyCart}
#[derive(Debug)]
pub struct UpdateDeliveryAddress{pub order_id: OrderId, pub delivery_address: DeliveryAddress}
#[derive(Debug)]
pub struct PayOrder{pub order_id: OrderId, pub payment_token: PaymentToken}

