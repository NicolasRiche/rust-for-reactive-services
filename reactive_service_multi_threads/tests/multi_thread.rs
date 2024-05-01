#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI64, Ordering};
    use std::time::Instant;
    use rayon::prelude::*;

    use reactive_service_domain::non_empty_cart::{NonEmptyCart, Quantity, Sku};
    use reactive_service_multi_threads::infra::postgres_events_store::PostgresEventStore;
    use reactive_service_multi_threads::order_service::{OrderService, UpdateCart};
    use reactive_service_multi_threads::payment_processor::LocalPaymentProcessor;
    use reactive_service_multi_threads::shipping_calculator::LocalShippingCalculator;
    use reactive_service_multi_threads::tax_calculator::LocalTaxCalculator;

    #[test]
    fn bench_throughput() {

        let event_journal= PostgresEventStore::new("postgresql://localhost").unwrap();

        let service = OrderService::new(
            event_journal,
            LocalShippingCalculator{},
            LocalTaxCalculator{},
            LocalPaymentProcessor{}
        );

        {
            // cycle over X entities
            let mut ring_iterator = (0i64..=1000i64).cycle();
            // warmup entities
            for _i in 0..1000 {
                let _ = service.update_cart(UpdateCart {
                    order_id: ring_iterator.next().unwrap(),
                    cart: NonEmptyCart::new(HashMap::from([(Sku("apple".to_owned()), Quantity(1))])).unwrap()
                });
            }
        }

        {
            let num_commands = 10000;
            let number_entities = 1000;

            let counter = Arc::new(AtomicI64::new(0));

            let start_time = Instant::now();

            (0..num_commands).into_par_iter().for_each(|_| {
                let order_id = counter.fetch_add(1, Ordering::SeqCst);
                if order_id > number_entities {
                    counter.swap(1, Ordering::SeqCst);
                }

                let _ = service.update_cart(UpdateCart {
                    order_id: order_id,
                    cart: NonEmptyCart::new(HashMap::from(
                    [
                        (Sku("apple".to_owned()), Quantity(1)),
                        (Sku("chocolate".to_owned()), Quantity(2))
                    ]
                    )).unwrap()
                });
            });

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
