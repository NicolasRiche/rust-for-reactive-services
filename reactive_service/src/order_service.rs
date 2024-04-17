use std::collections::HashMap;
use reactive_service_domain::aggregate_root::{AggregateRoot, SequencedEvent};
use reactive_service_domain::non_empty_cart::NonEmptyCart;
use reactive_service_domain::order_entity::{OrderCommand, OrderEntity, OrderEvent, OrderServices};
use reactive_service_domain::order_state::{DeliveryAddress, Money, OrderState};
use reactive_service_domain::shipping_calculator::{LocalShippingCalculator};
use reactive_service_domain::tax_calculator::{LocalTaxCalculator};

pub trait EventsStore<Event> {
    fn persist_event(&mut self, aggregate_id: i64, evt_w_seq: &SequencedEvent<Event>) -> Result<(), &'static str>;
    fn retrieve_events(&mut self, aggregate_id: i64) -> Result<Vec<SequencedEvent<Event>>, &'static str>;
}

pub struct OrderDependenciesServices {
    pub shipping_calculator: LocalShippingCalculator,
    pub tax_calculator: LocalTaxCalculator
}

impl OrderServices for OrderDependenciesServices {
    fn shipping_cost(&self, cart: &NonEmptyCart, delivery_address: &DeliveryAddress) -> Money {
        todo!()
    }

    fn tax_cost(&self, cart: &NonEmptyCart, shipping_cost: &Money) -> Money {
        todo!()
    }
}

pub struct OrderService<
    E: EventsStore<OrderEvent>,
> {
    orders: HashMap<i64, OrderEntity>,
    events_store: E,
    order_services: OrderDependenciesServices
}

impl <E> OrderService<E>
where E: EventsStore<OrderEvent>{

    pub fn new(events_store: E) -> Self {
        Self {
            orders: HashMap::default(),
            events_store,
            order_services: OrderDependenciesServices {
                shipping_calculator: LocalShippingCalculator{},
                tax_calculator: LocalTaxCalculator{}
            }
        }
    }

    fn dispatch_command(&mut self, entity_id: i64, command: OrderCommand)
        -> Result<(&OrderState, Vec<SequencedEvent<OrderEvent>>), &'static str> {

       let order = self.orders.entry(entity_id)
           .or_insert({
                let events = self.events_store.retrieve_events(entity_id).unwrap();
                let mut entity = OrderEntity::default();
                let _ = entity.restore_from_events(events)?;
                entity
            });

        let (state, events) = order.handle_command(command, &self.order_services)?;

        for evt in &events {
            self.events_store.persist_event(entity_id, evt)?;
        }
        Ok((state, events))
    }
}

