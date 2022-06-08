/// Utility to generate a binary number with the bit x set to 1
/// eg. bit!(2) == 0b100
#[macro_export]
macro_rules! bit {
    ( $($x:expr ), * ) => {
        $((2 as usize).pow($x))*
    };
}
