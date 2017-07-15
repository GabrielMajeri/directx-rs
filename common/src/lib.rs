//! Internal crate that `directx-rs` crates depend on.
#![cfg(windows)]
#![deny(warnings)]

extern crate comptr;
pub use comptr::ComPtr;

extern crate winapi as _winapi;

// Only reexport the modules we need.
pub mod winapi {
	pub use _winapi::Interface;

	pub use _winapi::shared::windef::HWND;
	pub use _winapi::um::unknwnbase::IUnknown;

	pub use _winapi::shared::winerror;

	// DXGI dependencies.
	pub use _winapi::shared::{dxgiformat, dxgitype, dxgi, dxgi1_2};

	// Direct3D 11 dependencies.
	pub use _winapi::um::{d3dcommon, d3d11};

	// Direct2D dependencies.
	pub use _winapi::um::{d2d1};
}

#[macro_use] mod object;
pub use object::DXObject;
