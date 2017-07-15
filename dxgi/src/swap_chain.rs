use {ComPtr, Interface, dxgi, dxgi1_2, dxgiformat, dxgitype, HWND, IUnknown};
use common::DXObject;

use factory::Factory;

use std::ptr;

/// Contains one or more surfaces for storing rendered data before presenting it to an output.
///
/// See the [IDXGISwapChain](https://msdn.microsoft.com/en-us/library/windows/desktop/bb174569(v=vs.85).aspx) interface documentation for more.
pub struct SwapChain(ComPtr<dxgi1_2::IDXGISwapChain1>);

impl SwapChain {
	/// Creates a new swapchain for a window.
	// TODO: is there a way to get a Device instead of an IUnknown?
	// TODO: maybe support creating swapchains for CoreWindow or Composition?
	// TODO: support creation options.
	pub fn new(factory: &Factory, device: &mut IUnknown, window: HWND) -> Self {
		let swap_chain = ComPtr::new({
			let desc = dxgi1_2::DXGI_SWAP_CHAIN_DESC1 {
				Width: 0,
				Height: 0,
				Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
				Stereo: false as i32,
				SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
					// No antialiasing.
					Count: 1,
					Quality: 0
				},
				BufferUsage: dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
				// Single buffering.
				BufferCount: 1,
				Scaling: dxgi1_2::DXGI_SCALING_STRETCH,
				// TODO: investigate the flip model.
				SwapEffect: dxgi::DXGI_SWAP_EFFECT_DISCARD,
				AlphaMode: dxgi1_2::DXGI_ALPHA_MODE_UNSPECIFIED,
				Flags: 0
			};

			let fullscreen_desc = dxgi1_2::DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
				RefreshRate: dxgitype::DXGI_RATIONAL {
					Numerator: 60,
					Denominator: 1
				},
				ScanlineOrdering: dxgitype::DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
				Scaling: dxgitype::DXGI_MODE_SCALING_UNSPECIFIED,
				Windowed: true as i32
			};

			let mut swap_chain = ptr::null_mut();

			let result = unsafe {
				factory.as_inner().CreateSwapChainForHwnd(
					device,
					window,
					&desc,
					&fullscreen_desc,
					// TODO: support restriction to output.
					ptr::null_mut(),
					&mut swap_chain
				)
			};

			assert_eq!(result, 0);

			swap_chain
		});

		SwapChain(swap_chain)
	}

	/// Returns a given buffer of the swap chain.
	pub fn get_buffer<I>(&self, index: u32) -> ComPtr<I>
		where I: Interface {
		// TODO: fix to check for number. of buffers.

		let mut view = ptr::null_mut();

		let result = unsafe {
			self.0.GetBuffer(
				// Get the back buffer.
				0,
				&I::uuidof(),
				&mut view
			)
		};

		assert_eq!(result, 0, "Failed to get buffer #{} from swap chain.", index);

		ComPtr::new(view as *mut _)
	}

	/// Presents a rendered image to the user.
	///
	/// https://msdn.microsoft.com/en-us/library/windows/desktop/bb174576(v=vs.85).aspx
	// TODO: support flags
	pub fn present(&self) {
		let _result = unsafe {
			self.0.Present(0, 0)
		};
	}
}
