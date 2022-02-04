//! # redux - A Rust implementation of Redux.
//!
//! Redux provides a clean way of managing states in an application.
//! It could be user data such as preferences or information about the state of the program.
//!
//! ## Concepts
//!
//! In Redux data is immutable. The only way to change it is to take it and create some new data by following a set of rules.
//!
//! ### State
//!
//! A state is the form of data Redux manages. Theoretically it can be anything, but for an easy explanation let's take the following example:
//! We have a simple counter application. It does nothing more than counting.
//! Our state would look the following:
//!
//! ```
//! #[derive(Default)]
//! struct State {
//!     counter: i8
//! }
//! ```
//!
//! ### Actions
//!
//! To change the state we need to dispatch actions. In Rust, they would usually be represented by an enum.
//! For the counter, we want to increment and decrement it.
//!
//! ```
//! enum Action {
//!     Increment,
//!     Decrement
//! }
//! ```
//!
//! ### Reducer
//!
//! To actually change the state (read: create a new one), we need what is called a reducer.
//! It is a simple function which takes in the current state plus the action to perform and returns a new state.
//!
//!
//! ### Store
//!
//! To put it all together, we use a store which keeps track of a state and provides an easy to use API for dispatching actions.
//! The store takes the reducer and an initial state.
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![feature(no_coverage)]

mod store;
pub use store::Store;

mod action;
pub use action::{ActionId, ActionWithMeta, EnablingCondition};

mod safety_condition;
pub use safety_condition::SafetyCondition;

mod reducer;
pub use reducer::Reducer;

mod effects;
pub use effects::Effects;

mod service;
pub use service::TimeService;
