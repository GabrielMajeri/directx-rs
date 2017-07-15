use {ComPtr, Interface, dxgi, dxgi1_2, winerror};
//use winapi::HWND;
use std::ptr;
use adapter::Adapter;

// Use this type when referring to the contained interface.
// This allows to change the interface's version without rewriting all the references.
type FactoryInterface = dxgi1_2::IDXGIFactory2;

/// Interface used to create other DXGI objects.
///
/// Can also be associated with a window to allow users to toggle fullscreen mode using Alt-Enter.
#[derive(Debug, Clone)]
pub struct Factory(ComPtr<FactoryInterface>);

impl Factory {
	/// Creates a new DXGI factory.
	pub fn new() -> Result<Factory, FactoryCreateError> {
		let factory = ComPtr::new({
			let mut factory = ptr::null_mut();

			let result = unsafe {
				// https://msdn.microsoft.com/en-us/library/windows/desktop/bb204862(v=vs.85).aspx
				// TODO: support using the debugging layer.
				dxgi::CreateDXGIFactory1(
					&FactoryInterface::uuidof(),
					&mut factory
				)
			};

			match result {
				winerror::S_OK => factory as *mut _,
				winerror::E_OUTOFMEMORY => return Err(FactoryCreateError::OutOfMemory),
				_ => return Err(FactoryCreateError::UnknownError(result))
			}
		});

		Ok(Factory(factory))
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

implement_object!(Factory, FactoryInterface);

/// Possible errors returned by [`Factory::new`](struct.Factory.html#method.new).
#[derive(Debug, Copy, Clone)]
pub enum FactoryCreateError {
	/// No memory available to create a new factory.
	OutOfMemory,
	/// Some unknown error.
	UnknownError(winerror::HRESULT)
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
	factory: &'factory FactoryInterface,
	index: u32
}

impl<'f> Iterator for AdapterIterator<'f> {
	type Item = Adapter;

	fn next(&mut self) -> Option<Self::Item> {
		let mut adapter = ptr::null_mut();

		let result = unsafe {
			self.factory.EnumAdapters1(self.index, &mut adapter)
		};

		if result == 0 {
			// No error, enumerated successfully.
			self.index += 1;
			Some(Adapter::new(ComPtr::new(adapter)))
		} else {
			None
		}
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
