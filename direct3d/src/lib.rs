//! Rust wrapper for the Direct3D API.

#![cfg(windows)]
#![deny(warnings, missing_docs)]

#[macro_use] extern crate common;
use common::ComPtr;
use common::winapi::{IUnknown, d3dcommon, d3d11};


extern crate dxgi;

/// Represents a logical device, an interface used to create device-specific resources.
pub mod device;

///
pub mod render_target_view;
