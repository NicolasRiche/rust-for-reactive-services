#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Instant;

    use reactive_service_domain::non_empty_cart::{NonEmptyCart, Quantity, Sku};
    use reactive_service_async::infra::postgres_events_store::PostgresEventStore;
    use reactive_service_async::order_service::{OrderService, UpdateCart};
    use reactive_service_async::payment_processor::LocalPaymentProcessor;
    use reactive_service_async::shipping_calculator::LocalShippingCalculator;
    use reactive_service_async::tax_calculator::LocalTaxCalculator;

    #[tokio::test(flavor = "current_thread")]
    async fn bench_throughput() {

        let event_journal= PostgresEventStore::new().await.unwrap();
        let mut service = OrderService::new(
            event_journal,
            LocalShippingCalculator{},
            LocalTaxCalculator{},
            LocalPaymentProcessor{}
        );

        let number_entities = 1000;

        {
            // cycle over X entities
            let mut ring_iterator = (0i64..=number_entities).cycle();
            // warmup entities
            for _i in 0..1000 {
                let _ = service.update_cart(UpdateCart {
                    order_id: ring_iterator.next().unwrap(),
                    cart: NonEmptyCart::new(HashMap::from([(Sku("apple".to_owned()), Quantity(1))])).unwrap()
                }).await;
            }
        }

        {
            let num_commands = 10000;

            // cycle over the entities
            let mut ring_iterator = (0..=number_entities).cycle();
            let start_time = Instant::now();

            for _i in 0..num_commands {
                let order_id = ring_iterator.next().unwrap();
                let _ = service.update_cart(UpdateCart {
                    order_id,
                    cart: NonEmptyCart::new(HashMap::from(
                        [
                            (Sku("apple".to_owned()), Quantity(1)),
                            (Sku("chocolate".to_owned()), Quantity(2))
                        ]
                    )).unwrap()
                }).await;
            }

            let elapsed_time = start_time.elapsed();
            let commands_per_sec = num_commands as f64 / elapsed_time.as_secs_f64();
            println!("Commands/seq {:?}", human_readable_format(commands_per_sec));
        }

    }

    fn human_readable_format(n: f64) -> String {
        if n < 1_000.0 {
            return format!("{:.2}", n);
        } else if n < 1_000_000.0 {
            return format!("{:.2}k", n / 1_000.0);
        } else if n < 1_000_000_000.0 {
            return format!("{:.2}M", n / 1_000_000.0);
        } else {
            return format!("{:.2}G", n / 1_000_000_000.0);
        }
    }
}
