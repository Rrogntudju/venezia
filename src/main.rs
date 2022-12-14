#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

const DELAI: u16 = 3600; // Une heure de fonctionnement
const DELAI_TEST: u16 = 10;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut crydom = pins.d12.into_output_high(); // Brancher l'alimentation
    let test = pins.d8.into_pull_up_input();
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let voltmètre = pins.a0.into_analog_input(&mut adc);
    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    arduino_hal::delay_ms(250); // Délai pour constante de temps RC
    let seuil = (voltmètre.analog_read(&mut adc) + 30).clamp(0, 1023);
    ufmt::uwriteln!(&mut serial, "Seuil: {}\r", seuil).void_unwrap();
    let mut délai: u16 = if test.is_high() { DELAI } else { DELAI_TEST };

    loop {
        let lecture = voltmètre.analog_read(&mut adc);
        if lecture > seuil {
            if led.is_set_low() {
                ufmt::uwriteln!(&mut serial, "Lecture: {}\r", lecture).void_unwrap();
            }
            led.set_high();
            délai = délai.saturating_sub(1);
            if délai == 0 {
                crydom.set_low(); // Couper l'alimentation
                loop {
                    led.toggle();
                    arduino_hal::delay_ms(250);
                }
            }
        } else {
            led.set_low();
            délai = if test.is_high() { DELAI } else { DELAI_TEST };
            crydom.set_high();
        }
        arduino_hal::delay_ms(1000);
    }
}
