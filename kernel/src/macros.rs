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
        use core::fmt::Write;
        $crate::devices::Devices::get().tty.write_fmt(format_args!($($arg)*)).unwrap()
    )
}

#[macro_export]
macro_rules! println {
    () => ($crate::term::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n",format_args!($($arg)*)))
}