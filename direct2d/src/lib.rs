//! Rustic wrapper around the Direct2D graphics API.

#![cfg(windows)]
#![deny(warnings, missing_docs)]

extern crate common;
use common::ComPtr;
use common::winapi::{d2d1, Interface};

use std::ptr;

/// Interface used to create other D2D objects.
pub struct Factory(ComPtr<d2d1::ID2D1Factory>);

impl Factory {
	/// Creates a new factory.
	pub fn new() -> Self {
		let factory = ComPtr::<d2d1::ID2D1Factory>::new({
			let options = d2d1::D2D1_FACTORY_OPTIONS {
				// TODO: support debug output.
				debugLevel: d2d1::D2D1_DEBUG_LEVEL_NONE
			};

			let mut factory = ptr::null_mut();

			let result = unsafe {
				d2d1::D2D1CreateFactory(
					// See https://msdn.microsoft.com/en-us/library/windows/desktop/dd368104(v=vs.85).aspx
					// TODO: multithreading
					d2d1::D2D1_FACTORY_TYPE_SINGLE_THREADED,
					&d2d1::ID2D1Factory::uuidof(),
					&options,
					&mut factory
				)
			};

			// TODO: error handling.
			assert_eq!(result, 0);

			factory as *mut _
		});

		Factory(factory)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_d2d_create() {
		let _factory = Factory::new();
	}
}
