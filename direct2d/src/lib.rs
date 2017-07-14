//! Rustic wrapper around the Direct2D graphics API.

#![cfg(windows)]
#![deny(warnings, missing_docs)]

extern crate comptr;
use comptr::ComPtr;

extern crate winapi;
use winapi::um::d2d1;

/// Interface used to create other D2D objects.
pub struct Factory(ComPtr<d2d1::ID2D1Factory>);

impl Factory {
	/// Creates a new factory.
	pub fn new() -> Self {
		let factory = ComPtr::new_with(|factory| {
			use std::mem;

			let options = d2d1::D2D1_FACTORY_OPTIONS {
				// TODO: support debug output.
				debugLevel: d2d1::D2D1_DEBUG_LEVEL_NONE
			};

			let result = unsafe {
				use winapi::Interface;
				d2d1::D2D1CreateFactory(
					// See https://msdn.microsoft.com/en-us/library/windows/desktop/dd368104(v=vs.85).aspx
					// TODO: multithreading
					d2d1::D2D1_FACTORY_TYPE_SINGLE_THREADED,
					&d2d1::ID2D1Factory::uuidof(),
					&options,
					mem::transmute(factory)
				)
			};

			assert_eq!(result, 0);
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
