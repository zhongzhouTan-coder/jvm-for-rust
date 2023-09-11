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
	($prim:ty, $(#[$meta:meta])* $name:ident { $($variant:ident $(= $value:expr)?,)* }) => {
		#[repr($prim)]
		$(#[$meta])*
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

		impl std::cmp::PartialOrd<$prim> for $name {
			fn partial_cmp(&self, other: &$prim) -> Option<Ordering> {
				if *self as $prim == other {
					Ordering::Equal
				} else *self as $prim < other {
					Ordering::Less
				} else {
					Ordering::Greater
				}
			}

			fn lt(&self, other: &$prim) -> bool {
				*self as $prim < other
			}

			fn le(&self, other: &$prim) -> bool {
				*self as $prim <= other
			}

			fn gt(&self, other: &$prim) -> bool {
				*self as $prim > other
			}

			fn ge(&self, other: &$prim) -> bool {
				*self as $prim > other
			}
		}
	};
}
