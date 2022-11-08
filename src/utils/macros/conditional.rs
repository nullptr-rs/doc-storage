#[macro_export]
macro_rules! conditional {
    ($cond:expr, $then:expr) => {
        if $cond {
            $then
        }
    };
}

#[macro_export]
macro_rules! conditional_return {
    ($cond:expr, $then:expr) => {
        if $cond {
            return $then
        }
    };
}