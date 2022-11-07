#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut crydom = pins.d12.into_output_high();   // Brancher l'alimentation
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let voltmètre = pins.a0.into_analog_input(&mut adc);
    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    arduino_hal::delay_ms(100);
    let seuil = (voltmètre.analog_read(&mut adc) + 10).clamp(0, 1023);
    ufmt::uwriteln!(&mut serial, "Seuil: {}\r", seuil).void_unwrap();
    let mut délai: u16 = 3600;

    loop {
        arduino_hal::delay_ms(200);
        let lecture = voltmètre.analog_read(&mut adc);
        if lecture > seuil {
            led.set_high();
            délai = délai.saturating_sub(1);
            if délai == 0 {
                crydom.set_low();   // Couper l'alimentation après une heure de fonctionnement
            }

        } else {
            led.set_low();
            délai = 3600;
            crydom.set_high();
        }
        arduino_hal::delay_ms(800);
    }

}
