#[macro_export]
macro_rules! align_up {
    ($size:expr, $align:expr) => {
        ($size + $align - 1) & !($align - 1)
    };
}