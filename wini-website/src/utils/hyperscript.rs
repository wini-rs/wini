#[macro_export]
macro_rules! hs {
    ($($tokens:tt)*) => {
        (maud::PreEscaped(stringify!($($tokens)*).replace('\n', " ").replace('.', " .")))
    };
}
