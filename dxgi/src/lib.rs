//! Rust wrapper around the DirectX Graphics Infrastructure library.
//!
//! The entry point for most of the DXGI functions is the [`Factory`](factory/struct.Factory.html) interface.
//!
//! For information on how to work with DXGI, read [the official DXGI documentation](https://msdn.microsoft.com/en-us/library/windows/desktop/bb205075(v=vs.85).aspx).

#![cfg(windows)]
#![deny(warnings, missing_docs)]

#[macro_use] extern crate common;
use common::ComPtr;
use common::winapi::{dxgi, dxgi1_2, dxgiformat, dxgitype, Interface, HWND, IUnknown};

// TODO: investigate the thread safety of DXGI. The documentation is unclear.

/// A factory is used to generate other DXGI objects.
pub mod factory;
/// An adapter represents a graphics subsystem.
pub mod adapter;
/// A swap chain is a collection of buffers used for displaying images.
pub mod swap_chain;
