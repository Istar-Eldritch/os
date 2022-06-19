/// Utility to generate a binary number with the bit x set to 1
/// eg. bit!(2) == 0b100
#[macro_export]
macro_rules! bit {
    ( $($x:expr ), * ) => {
        $((2 as usize).pow($x))*
    };
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (
        $crate::devices::Devices::get().tty.print(format_args!($($arg)*))
    )
}

#[macro_export]
macro_rules! println {
    () => ($crate::term::print!("\n\r"));
    ($($arg:tt)*) => ($crate::print!("{}\n\r",format_args!($($arg)*)))
}