# Rust for Reactive services - Course code samples

These are the samples for the course "Rust for Reactive services".

- [slides_support](slides_support/) :
Illustrate how Rust help to increase Compile time correctness.
  - Make invalid state unrepresentable (and with zero-cost abstractions) 
  - Expose Type safe APIs
  - Leverage Ownership system to enforce business logic

The second part of the course is an exploration of using Rust to implement a e-commerce order service.
Using event-sourcing and following the onion architecture.

- Starting with the domain [reactive_service_domain](reactive_service_domain/):
  - How to model an order state and the associated entity. Exposing them with a type safe finite state machine.

- Then, the application layer, going through different concurrency strategies
  - [reactive_service_single_thread](reactive_service_single_thread/)
  - [reactive_service_multi-threads](reactive_service_multi-threads/)
  - [reactive_service_async](reactive_service_async/)