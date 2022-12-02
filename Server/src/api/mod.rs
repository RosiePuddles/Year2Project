//! # API module
//!
//! This module includes the [paths](paths) and [data types](prelude) used by the API.
//!
//! All of the paths have test modules under the `tests` directory
mod impls;
pub mod paths;
pub mod prelude;
#[cfg(test)]
mod tests;
