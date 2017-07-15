use {ComPtr, d3dcommon, d3d11, IUnknown};
use common::DXObject;

use dxgi::adapter::Adapter;

use std::{mem, ptr};

/// Creates a new device.
// TODO: options
pub fn create_device(adapter: Option<&Adapter>) -> (Device, DeviceContext) {
	let adapter = match adapter {
		// D3D will pick the primary adapter of the system.
		None => ptr::null_mut(),
		// Creates a device for a specified graphics adapter.
		Some(adapter) => unsafe { mem::transmute(adapter.as_inner().as_mut_ptr()) }
	};

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

	let (mut device, mut device_context) = (ptr::null_mut(), ptr::null_mut());

	let result = unsafe {
		// See https://msdn.microsoft.com/en-us/library/windows/desktop/ff476082(v=vs.85).aspx for reference.
		d3d11::D3D11CreateDevice(
			adapter,
			d3dcommon::D3D_DRIVER_TYPE_UNKNOWN,
			// TODO: support software renderer.
			ptr::null_mut(),
			// See https://msdn.microsoft.com/en-us/library/windows/desktop/ff476107(v=vs.85).aspx for a list of flags.
			// TODO: support the debug layer.
			d3d11::D3D11_CREATE_DEVICE_DEBUG,
			feature_levels.as_ptr(),
			feature_levels.len() as u32,
			// According to the docs, this should always be set to this value.
			d3d11::D3D11_SDK_VERSION,
			&mut device,
			// TODO: return the feature level.
			ptr::null_mut(),
			&mut device_context
		)
	};

	assert_eq!(result, 0);

	let device = Device(ComPtr::new(device));
	let device_context = DeviceContext(ComPtr::new(device_context));

	(device, device_context)
}

type DeviceInterface = d3d11::ID3D11Device;

/// Read the official MSDN documentation for the
/// [ID3D11Device](https://msdn.microsoft.com/en-us/library/windows/desktop/ff476379(v=vs.85).aspx) interface.
pub struct Device(ComPtr<DeviceInterface>);

impl Device {
	/// Up-casts this device to the `IUnknown` interface. Useful when working with DXGI.
	pub fn as_unknown(&self) -> &mut IUnknown {
		unsafe {
			mem::transmute(self.as_inner().as_mut_ptr())
		}
	}
}

implement_object!(Device, DeviceInterface);

type DeviceContextInterface = d3d11::ID3D11DeviceContext;

/// Used to send commands to the GPU.
pub struct DeviceContext(ComPtr<DeviceContextInterface>);

impl DeviceContext {
	/// Sets a render target to a given color.
	// TODO: support custom clear color.
	pub fn clear_render_target_view(&self, rt_view: &RenderTargetView) {
		let color = [0.0, 0.0, 0.0, 0.0];

		unsafe {
			self.as_inner().ClearRenderTargetView(
				rt_view.as_inner().as_mut_ptr(),
				&color
			)
		}
	}
	/// Returns a structure used to change the output-merger stage state.
	pub fn output_merger(&self) -> OutputMerger {
		OutputMerger(&self.0)
	}
}

implement_object!(DeviceContext, DeviceContextInterface);

use render_target_view::RenderTargetView;

/// See https://msdn.microsoft.com/en-us/library/windows/desktop/bb205120(v=vs.85).aspx
pub struct OutputMerger<'dc>(&'dc ComPtr<d3d11::ID3D11DeviceContext>);

impl<'dc> OutputMerger<'dc> {
	/// See https://msdn.microsoft.com/en-us/library/windows/desktop/ff476464(v=vs.85).aspx
	pub fn set_render_targets(&self, rt_views: Option<&[&RenderTargetView]>) {
		let count = rt_views.map_or(0, |rt_views| rt_views.len());
		let rt_views = rt_views.map_or(ptr::null_mut(), |rt_views| unsafe { mem::transmute(*rt_views.as_ptr()) });

		unsafe {
			self.0.OMSetRenderTargets(
				count as u32,
				rt_views,
				// TODO: allow setting depth/stencil views.
				ptr::null_mut()
			);
		}
	}
}
