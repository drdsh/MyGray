//! Typed repositories, one per aggregate. Each has a trait (for mocking in
//! other items' tests) and a Grafeo implementation.

pub mod alert;
pub mod audit;
pub mod device;
pub mod dose;
pub mod patient;
pub mod study;
pub mod user;
