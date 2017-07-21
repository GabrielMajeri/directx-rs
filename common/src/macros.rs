/// Implements the traits required to convert an object to its raw representation.
/// `$object` is the name of the struct, e.g. `Factory`.
/// `$interface` is the name of the wrapped interface, e.g. `IDXGIFactory`.
#[macro_export]
macro_rules! implement_object {
	($object:ident, $interface:ident) => (
		impl ::std::convert::Into<$crate::ComPtr<$interface>> for $object {
			fn into(self) -> $crate::ComPtr<$interface> {
				self.0
			}
		}

		impl ::std::convert::AsRef<$crate::ComPtr<$interface>> for $object {
			fn as_ref(&self) -> &$crate::ComPtr<$interface> {
				&self.0
			}
		}
	)
}
