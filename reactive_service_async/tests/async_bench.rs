#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use reactive_service_domain::non_empty_cart::{NonEmptyCart, Quantity, Sku};
    use tokio::sync::Semaphore;
    use reactive_service_async::infra::postgres_events_store::PostgresEventStore;
    use reactive_service_async::infra::scylla_event_store::ScyllaEventStore;
    use reactive_service_async::order_service::{EventsJournal, OrderService, UpdateCart};
    use reactive_service_async::payment_processor::LocalPaymentProcessor;
    use reactive_service_async::shipping_calculator::LocalShippingCalculator;
    use reactive_service_async::tax_calculator::LocalTaxCalculator;
    use reactive_service_domain::order_entity::OrderEvent;

    #[tokio::test(flavor = "current_thread")]
    // #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn bench_postgres() {
        let events_journal = PostgresEventStore::new().await.unwrap();
        let max_concurrent_tasks = 10;
        bench_throughput(events_journal, max_concurrent_tasks).await
    }

    #[tokio::test(flavor = "current_thread")]
    async fn bench_scylla() {
        // Note: to run a single node Scylla
        // docker run --rm -it -p 9042:9042 scylladb/scylla
        let events_journal = ScyllaEventStore::new("127.0.0.1:9042").await.unwrap();
        let max_concurrent_tasks = 200;
        bench_throughput(events_journal, max_concurrent_tasks).await
    }
    async fn bench_throughput<E: EventsJournal<OrderEvent> + Send + Sync + 'static>(
        events_journal: E, max_concurrent_tasks: usize) {

        let service = Arc::new(OrderService::new(
            events_journal,
            LocalShippingCalculator{},
            LocalTaxCalculator{},
            LocalPaymentProcessor{}
        ));
        
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
            let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));

            // cycle over the entities
            let mut ring_iterator = (0..=number_entities).cycle();
            let start_time = Instant::now();

            let mut handles = Vec::new();

            for _i in 0..num_commands {

                let order_id = ring_iterator.next().unwrap();

                match semaphore.clone().acquire_owned().await {
                    Ok(permit) => {
                        let cmd = UpdateCart {
                            order_id,
                            cart: NonEmptyCart::new(HashMap::from(
                                [
                                    (Sku("apple".to_owned()), Quantity(1)),
                                    (Sku("chocolate".to_owned()), Quantity(2))
                                ]
                            )).unwrap()
                        };

                        // We need a clone of the Arc, because 'tokio::spawn(async move' will take ownership
                        let service_arc = service.clone();

                        let handle = tokio::spawn(async move {
                            let _ = service_arc.update_cart(cmd).await;
                            drop(permit);
                        });

                        handles.push(handle);
                    }

                    _ => {
                        tokio::time::sleep(Duration::from_micros(10)).await;
                    }
                }
            }

            for handle in handles {
                handle.await.expect("Task panicked");
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
