//! `std::panic::set_hook` that converts panics (with location, thread name) into Critical alerts (SRS-ALR-002), then chains to the default hook.
