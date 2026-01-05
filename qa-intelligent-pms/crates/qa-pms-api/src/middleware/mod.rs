//! Middleware for request processing.
//!
//! Contains middleware functions for enhancing request handling,
//! including request ID correlation, tracing, and other cross-cutting concerns.

pub mod request_id;

pub use request_id::request_id_middleware;
