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

#[macro_export]
macro_rules! ternary_as {
    ($cond:expr, $then:expr, $else:expr, $as:ty) => {
        if $cond { $then } else { $else } as $as
    };
}
