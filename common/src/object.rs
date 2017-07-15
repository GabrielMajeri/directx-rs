use comptr::ComPtr;

/// Trait implemented by all structs that wrap DirectX interfaces.
pub trait DXObject<I> {
	/// Returns the raw `ComPtr` stored in this interface.
	fn into_inner(self) -> ComPtr<I>;

	/// Returns a reference to the `ComPtr` holding the interface.
	fn as_inner(&self) -> &ComPtr<I>;
}

/// Implements the `DXObject` interface.
/// `$object` is the name of the struct, e.g. `Factory`.
/// `$interface` is the name of the wrapped interface, e.g. `IDXGIFactory`.
#[macro_export]
macro_rules! implement_object {
	($object:ident, $interface:ident) => (
		impl $crate::DXObject<$interface> for $object {
			fn into_inner(self) -> ComPtr<$interface> {
				self.0
			}

			fn as_inner(&self) -> &ComPtr<$interface> {
				&self.0
			}
		}
	)
}
