#[macro_export]
macro_rules! conditional {
    ($cond:expr, $then:expr) => {
        if $cond {
            $then
        }
    };
}

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
