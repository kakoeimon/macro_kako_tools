#[macro_export]
macro_rules! ka_layer {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_i32: i32 = 0;
            $(
                temp_i32 |= 1 << $x;
            )*
            temp_i32
        }
    };
}