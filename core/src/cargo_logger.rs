#[macro_export]
macro_rules! warn {
    // warn!("a {} event", "log")
    ($($arg:tt)+) => (println!("cargo:warning={}",format_args!($($arg)+)))
}
