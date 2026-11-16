//! # mygray-web — Software item SI-07
//!
//! Simple HTTP server with **server-side rendering (SSR)**: every page is
//! rendered on the server with compile-time-checked askama templates. No
//! client-side framework, no WASM; optional tiny progressive enhancement
//! (`static/app.js`, charts as server-rendered SVG). Rationale (ADR-0005):
//! smaller attack surface, no SOUP in the browser, deterministic output that
//! can be snapshot-tested (insta) — important for verifying that dose values
//! and units are displayed correctly (SRS-WEB-003, HAZ-004).
//!
//! The web layer is **read-mostly**: only alert acknowledgement, rule
//! editing, user admin and configuration go through POST + CSRF token.

pub mod auth;
pub mod error;
pub mod routes;
pub mod security;
pub mod server;
pub mod state;
pub mod views;
