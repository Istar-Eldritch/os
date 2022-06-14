use crate::hifive::*;
use crate::drivers::gpio::Gpio;

// This should take ownership of the specific GPIO pins
pub struct Leds {
    gpio: Gpio,
    red: bool,
    green: bool,
    blue: bool,
}

impl Leds {
    pub fn new(gpio: Gpio) -> Self {
        Self {
            gpio,
            red: false,
            green: false,
            blue: false
        }
    }

    pub fn set_red(&mut self, value: bool) {
        self.red = value;
        self.gpio.output_val().set_pin22(if value { 1 } else { 0 });
    }

    pub fn set_green(&mut self, value: bool) {
        self.green = value;
        self.gpio.output_val().set_pin19(if value { 1 } else { 0 });
    }

    pub fn set_blue(&mut self, value: bool) {
        self.blue = value;
        self.gpio.output_val().set_pin21(if value { 1 } else { 0 });
    }

    pub fn init(&mut self) {
        // Enable the leds
        self.gpio.output_en().set_all(LED_GREEN | LED_RED | LED_BLUE);
        self.gpio.out_xor().set_all(LED_GREEN | LED_RED | LED_BLUE);
    }
}
