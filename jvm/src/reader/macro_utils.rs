#[macro_export]
macro_rules! primitive_enum {
	($prim:ty, $(#[$meta:meta])* $name:ident { $($variant:ident $(= $value:expr)?,)* }) => {
		#[repr($prim)]
		$(#[$meta])*
		#[derive(Copy, Clone, PartialEq)]
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

		impl std::cmp::PartialEq<$prim> for $name {
			fn eq(&self, other: &$prim) -> bool {
				*self as $prim == *other
			}

    		fn ne(&self, other: &$prim) -> bool {
				*self as $prim != *other
			}
		}

		impl std::cmp::PartialOrd<$prim> for $name {
			fn partial_cmp(&self, other: &$prim) -> Option<std::cmp::Ordering> {
				if *self as $prim == *other {
					Some(std::cmp::Ordering::Equal)
				} else if *self as $prim < *other {
					Some(std::cmp::Ordering::Less)
				} else {
					Some(std::cmp::Ordering::Greater)
				}
			}

			fn lt(&self, other: &$prim) -> bool {
				*self as $prim < *other
			}

			fn le(&self, other: &$prim) -> bool {
				*self as $prim <= *other
			}

			fn gt(&self, other: &$prim) -> bool {
				*self as $prim > *other
			}

			fn ge(&self, other: &$prim) -> bool {
				*self as $prim > *other
			}
		}
	};
}
