//! Rust wrapper for the Direct3D API.

#![cfg(windows)]
#![deny(warnings, missing_docs)]

extern crate comptr;
extern crate winapi;
extern crate dxgi;

/// Represents a logical device, an interface used to create device-specific resources.
pub mod device;
