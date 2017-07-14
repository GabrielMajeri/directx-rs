use comptr::ComPtr;
use winapi::shared::{dxgi, dxgi1_2/*, winerror*/};
//use winapi::shared::windef::HWND;
use std::{mem/*, ptr*/};
use adapter::Adapter;

/// Interface used to create other DXGI objects.
///
/// Can also be associated with a window to allow users to toggle fullscreen mode using Alt-Enter.
#[derive(Debug, Clone)]
pub struct Factory(ComPtr<dxgi1_2::IDXGIFactory2>);

impl Factory {
	/// Creates a new DXGI factory.
	pub fn new() -> Factory {
		let factory = ComPtr::new_with(|ptr| {
			let result = unsafe {
				use winapi::Interface;

				// https://msdn.microsoft.com/en-us/library/windows/desktop/bb204862(v=vs.85).aspx
				// TODO: support using the debugging layer.
				dxgi::CreateDXGIFactory1(
					&dxgi1_2::IDXGIFactory2::uuidof(),
					mem::transmute(ptr)
				)
			};

			assert_eq!(result, 0);
		});

		Factory(factory)
	}

	/// Returns an iterator to all of the adapters on the system.
	///
	/// This function enumerates both adapters with and without outputs.
	pub fn adapters(&self) -> AdapterIterator {
		AdapterIterator {
			factory: &*self.0,
			index: 0
		}
	}

	// TODO: it looks like alt-enter works without a window association anyway. What's the point of this API then?
/*
	/// Associates a window with this factory.
	///
	/// This will fail if there is already a window associated with this factory.
	// TODO: support flags.
	pub fn associate_window(&self, window: HWND) -> Result<WindowAssociation, AssociateWindowError> {
		if self.get_associated_window() != ptr::null_mut() {
			Err(AssociateWindowError::AlreadyAssociated)
		} else {
			let result = unsafe {
				self.0.MakeWindowAssociation(window, 0)
			};

			match result {
				0 => Ok(WindowAssociation {
					factory: &self
				}),
				winerror::DXGI_ERROR_INVALID_CALL => Err(AssociateWindowError::InvalidHandle),
				winerror::E_OUTOFMEMORY => Err(AssociateWindowError::OutOfMemory),
				_ => panic!("Unknown error when creating swapchain.")
			}


		}
	}

	/// Returns the handle of the currently associated window.
	pub fn get_associated_window(&self) -> HWND {
		let mut window = ptr::null_mut();

		unsafe {
			self.0.GetWindowAssociation(&mut window);
		}

		window
	}
*/
}
/*
/// Errors returned by `Factory::associate_window`.
#[derive(Debug, Copy, Clone)]
pub enum AssociateWindowError {
	/// The given window handle is not valid.
	InvalidHandle,
	/// Another association still exists.
	AlreadyAssociated,
	/// DXGI ran out of memory.
	OutOfMemory
}

/// As long as this struct lives, the DXGI factory will monitor the message queue for a given window
/// and respond to `Alt` + `Enter` and the `Print Screen` keys.
#[must_use]
pub struct WindowAssociation<'factory> {
	factory: &'factory Factory
}

impl<'f> Drop for WindowAssociation<'f> {
	fn drop(&mut self) {
		unsafe {
			self.factory.0.MakeWindowAssociation(ptr::null_mut(), 0);
		}
	}
}
*/
/// Iterator for the adapters connected to the computer.
pub struct AdapterIterator<'factory> {
	factory: &'factory dxgi1_2::IDXGIFactory2,
	index: u32
}

impl<'f> Iterator for AdapterIterator<'f> {
	type Item = Adapter;

	fn next(&mut self) -> Option<Self::Item> {
		let adapter = ComPtr::try_new_with(|ptr| {
			let result = unsafe {
				self.factory.EnumAdapters1(self.index, ptr)
			};

			if result == 0 {
				// No error, enumerated successfully.
				self.index += 1;
				None
			} else {
				Some(result)
			}
		});

		adapter.ok()
			.map(Adapter::new)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create_factory() {
		let _factory = Factory::new();
	}
}
