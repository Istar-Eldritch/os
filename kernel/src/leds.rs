use crate::drivers::gpio::*;
use crate::hifive::*;

// TODO: Mutex this
static mut LEDS: Option<Leds>  = None;

#[derive(Clone)]
pub struct Leds {
    red: bool,
    green: bool,
    blue: bool,
}

impl Leds {
    pub fn set_red(&mut self, value: bool) {
        self.red = value;
        let gpio = GPIO::new(GPIO_ADDR);
        gpio.output_val().set_pin22(if value { 1 } else { 0 });
    }

    pub fn set_green(&mut self, value: bool) {
        self.green = value;
        let gpio = GPIO::new(GPIO_ADDR);
        gpio.output_val().set_pin19(if value { 1 } else { 0 });
    }

    pub fn set_blue(&mut self, value: bool) {
        self.blue = value;
        let gpio = GPIO::new(GPIO_ADDR);
        gpio.output_val().set_pin21(if value { 1 } else { 0 });
    }
}

pub fn init_leds() {
    let gpio = GPIO::new(GPIO_ADDR);

    // Enable the leds
    gpio.output_en().set_all(LED_GREEN | LED_RED | LED_BLUE);
    gpio.out_xor().set_all(LED_GREEN | LED_RED | LED_BLUE);
    
    unsafe { LEDS = Some(Leds { red: false, green: false, blue: false }) };
}

pub fn get_leds() -> Leds {
    unsafe { LEDS.clone().unwrap() }
}