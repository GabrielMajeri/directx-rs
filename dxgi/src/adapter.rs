use {ComPtr};
use dxgi;
use std::{mem, fmt};

type AdapterInterface = dxgi::IDXGIAdapter1;

/// Represents a display subsystem, such as a graphics card, or a software renderer.
///
/// Applications do not create adapters, rather they enumerate them using [`Factory::adapters`](struct.Factory.html#method.adapters).
///
/// Read the [IDXGIAdapter](https://msdn.microsoft.com/en-us/library/windows/desktop/bb174523(v=vs.85).aspx) interface documentation on MSDN for more information.
// TODO: use the IDXGIAdapter2 structure
#[derive(Debug, Clone)]
pub struct Adapter(ComPtr<AdapterInterface>);

impl Adapter {
	// TODO: this function is used by `Factory` to create these adapters.
	// Is there any workaround that does not require this function?
	pub (in super) fn new(adapter: ComPtr<AdapterInterface>) -> Self {
		Adapter(adapter)
	}

	/// Returns the adapter description.
	pub fn description(&self) -> AdapterDescription {
		let mut desc = unsafe { mem::uninitialized() };

		unsafe {
			self.0.GetDesc1(mem::transmute(&mut desc));
		}

		desc
	}
}

implement_object!(Adapter, AdapterInterface);

/// A structure contain information about the adapter and its capabilities.
#[derive(Copy, Clone)]
pub struct AdapterDescription(dxgi::DXGI_ADAPTER_DESC1);

impl AdapterDescription {
	/// Returns a human-readable description of the adapter.
	pub fn name(&self) -> String {
		String::from_utf16_lossy(&self.0.Description)
	}

	/// Returns the size in bytes of video memory not shared with the CPU.
	pub fn dedicated_video_memory(&self) -> usize {
		self.0.DedicatedVideoMemory
	}
}

impl fmt::Debug for AdapterDescription {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// TODO: finish debug trait implementation.
		write!(f, "AdapterDescription {{ {} }}", self.name())
	}
}
