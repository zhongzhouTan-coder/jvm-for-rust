//! A garbage collector for the JVM.
//!
//! A rust implementation for java virtual machine and reference to immix garbage
//! collector implementation that is a mark-region collector.

#![warn(missing_docs)]

mod allocator;
mod model;
mod utils;
