#![no_std]
#![no_main]

use arduino_hal::hal::port::PB5;
use arduino_hal::port::{mode::Output, Pin};
use panic_halt as _;

fn blink_code(d13: &mut Pin<Output, PB5>, n: usize) {
    for _ in 0..n {
        d13.toggle();
        arduino_hal::delay_ms(175);
        d13.toggle();
        arduino_hal::delay_ms(175);
    }
    arduino_hal::delay_ms(1000);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();

    loop {
        blink_code(&mut led, 3);
    }
}
