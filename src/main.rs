#![no_std]
#![no_main]

use arduino_hal::hal::port::PB5;
use arduino_hal::port::{mode::Output, Pin};
use panic_halt as _;

fn clignote_code(led: &mut Pin<Output, PB5>, n: usize) {
    for _ in 0..n {
        led.toggle();
        arduino_hal::delay_ms(175);
        led.toggle();
        arduino_hal::delay_ms(175);
    }
    arduino_hal::delay_ms(1000);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut led = pins.d13.into_output();
    let galvanomètre = pins.a0.into_analog_input(&mut adc);
    let crydom = pins.d12.into_output_high();   // Crydom à ON

    loop {
        let voltage = galvanomètre.analog_read(&mut adc);
        clignote_code(&mut led, 3);
    }
}
