use {ComPtr, d3d11};

use dxgi::swap_chain::SwapChain;

use device::Device;

use std::ptr;

type RTViewInterface = d3d11::ID3D11RenderTargetView;

///
pub struct RenderTargetView(ComPtr<RTViewInterface>);

impl RenderTargetView {
	/// Creates a RTV from the back buffer of a swap chain.
	// TODO: more ways to create render target view.
	pub fn from_swap_chain_back_buffer(device: &Device, swap_chain: &SwapChain) -> Self {
		let back_buffer = swap_chain.get_buffer::<d3d11::ID3D11Resource>(0);

		let mut rt_view = ptr::null_mut();

		let result = unsafe {
			device.as_ref().CreateRenderTargetView(
				back_buffer.get_mut(),
				ptr::null_mut(),
				&mut rt_view
			)
		};

		assert_eq!(result, 0, "Failed to create render target view.");

		RenderTargetView(ComPtr::new(rt_view))
	}
}

implement_object!(RenderTargetView, RTViewInterface);
