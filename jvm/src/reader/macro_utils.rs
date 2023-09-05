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
