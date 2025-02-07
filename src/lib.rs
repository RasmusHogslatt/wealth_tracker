#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod models;
mod services;
mod ui;
pub use app::WealthTrackerApp;
pub use models::*;
pub use services::*;
pub use ui::*;
