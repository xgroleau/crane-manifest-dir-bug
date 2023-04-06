//! Wrapper and build script for the protobuf messages
#![deny(missing_docs)]
#![allow(clippy::module_inception)]

/// hoptech.measurement namespace
pub mod example {
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
}
