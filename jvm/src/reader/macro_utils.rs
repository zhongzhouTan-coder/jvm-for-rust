#[macro_export]
macro_rules! switch {
    ($p:ident, $t:ty, { $($e:expr => $code:block)* $(_ => $d:block)? }) => {
		match $p {
			$(
				x if x == $e as $t => $code,
			)*
			$(
				_ => $d
			)?
		}
	};
}

#[macro_export]
macro_rules! primitive_enum {
	($prim:ty, $(#[$meta:meta])? $name:ident { $($variant:ident $(= $value:expr)?,)* }) => {
		#[repr($prim)]
		$(#[$meta])?
		pub enum $name {
			$($variant $(= $value)?),*
		}

	impl $name {
			pub fn from(value: $prim) -> Option<Self> {
				match value {
					$(x if x == $name::$variant as $prim => Some($name::$variant),)*
					_ => None,
				}
			}
		}
	};
}
