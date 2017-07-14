use comptr::ComPtr;

use winapi::um::{d3d11, d3dcommon};
use winapi::um::unknwnbase::IUnknown;

use dxgi::adapter::Adapter;

use std::{ptr, mem};

/// Read the official MSDN documentation for the
/// [ID3D11Device](https://msdn.microsoft.com/en-us/library/windows/desktop/ff476379(v=vs.85).aspx) interface.
pub struct Device(ComPtr<d3d11::ID3D11Device>);

impl Device {
	/// Creates a new device.
	// TODO: options
	pub fn new(_adapter: Option<Adapter>) -> Self {
		let device = ComPtr::new_with(move |ptr| {
/*			let adapter_ptr = match adapter {
				None => ptr::null_mut(),
				Some(adapter) => unsafe { mem::transmute(adapter) }
			};
*/
			// Require at least Direct3D 11.1.
			let feature_levels = [
				d3dcommon::D3D_FEATURE_LEVEL_11_1,
				d3dcommon::D3D_FEATURE_LEVEL_11_0,
				d3dcommon::D3D_FEATURE_LEVEL_10_1,
				d3dcommon::D3D_FEATURE_LEVEL_10_0,
				d3dcommon::D3D_FEATURE_LEVEL_9_3,
				d3dcommon::D3D_FEATURE_LEVEL_9_2,
				d3dcommon::D3D_FEATURE_LEVEL_9_1,
			];

			let result = unsafe {
				// See https://msdn.microsoft.com/en-us/library/windows/desktop/ff476082(v=vs.85).aspx for reference.
				d3d11::D3D11CreateDevice(
					ptr::null_mut(), //adapter_ptr,
					d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
					// TODO: support software renderer.
					ptr::null_mut(),
					// See https://msdn.microsoft.com/en-us/library/windows/desktop/ff476107(v=vs.85).aspx for a list of flags.
					0,
					feature_levels.as_ptr(),
					feature_levels.len() as u32,
					d3d11::D3D11_SDK_VERSION,
					ptr,
					// TODO: return the feature level.
					ptr::null_mut(),
					// TODO: also return immediate context.
					ptr::null_mut()
				)
			};

			assert_eq!(result, 0);
		});

		Device(device)
	}

	/// Up-casts this device to the `IUnknown` interface. Useful when working with DXGI.
	pub fn as_unknown(&self) -> &mut IUnknown {
		unsafe {
			mem::transmute(self.0.to_raw())
		}
	}
}
