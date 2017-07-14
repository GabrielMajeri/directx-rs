use comptr::ComPtr;

use winapi::shared::{dxgi, dxgi1_2, dxgiformat, dxgitype};
use winapi::shared::windef::HWND;
use winapi::um::unknwnbase::IUnknown;

use factory::Factory;

use std::mem;

/// Contains one or more surfaces for storing rendered data before presenting it to an output.
pub struct SwapChain(ComPtr<dxgi1_2::IDXGISwapChain1>);

impl SwapChain {
	/// Creates a new swapchain for a window.
	// TODO: maybe support creating swapchains for CoreWindow or Composition?
	// TODO: support creation options.
	pub fn new(factory: &Factory, device: &mut IUnknown, window: HWND) -> Self {
		let swap_chain = ComPtr::new_with(|ptr| {

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

			let result = unsafe {
				mem::transmute::<_, &ComPtr<dxgi1_2::IDXGIFactory2>>(factory).to_raw().CreateSwapChainForHwnd(
					device,
					window,
					&desc,
					&fullscreen_desc,
					// TODO: support restriction to output.
					::std::ptr::null_mut(),
					ptr
				)
			};

			assert_eq!(result, 0);
		});

		SwapChain(swap_chain)
	}
}
