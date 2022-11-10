#[macro_export]
macro_rules! ternary {
    ($cond:expr, $then:expr, $else:expr) => {
        if $cond {
            $then
        } else {
            $else
        }
    };
}
