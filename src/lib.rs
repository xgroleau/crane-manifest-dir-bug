//! Wrapper and build script for the protobuf messages
#![deny(missing_docs)]
#![allow(clippy::module_inception)]

/// hoptech.measurement namespace
pub mod measurement {
    include!(concat!(env!("OUT_DIR"), "/hoptech.measurement.rs"));
}

/// hoptech.notification namespace
pub mod notification {
    include!(concat!(env!("OUT_DIR"), "/hoptech.notification.rs"));
}

/// hoptech.request namespace
pub mod request {
    include!(concat!(env!("OUT_DIR"), "/hoptech.request.rs"));
}

/// hoptech.response namespace
pub mod response {
    include!(concat!(env!("OUT_DIR"), "/hoptech.response.rs"));
}

/// hoptech.session namespace
pub mod session {
    include!(concat!(env!("OUT_DIR"), "/hoptech.session.rs"));
}

/// hoptech.wearable namespace
pub mod wearable {
    include!(concat!(env!("OUT_DIR"), "/hoptech.wearable.rs"));
}
