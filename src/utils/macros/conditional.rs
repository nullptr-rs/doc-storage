#[macro_export]
macro_rules! conditional {
    ($cond:expr, $then:expr) => {
        if $cond {
            $then
        }
    };
}
