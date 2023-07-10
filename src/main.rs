#![no_std]
#![no_main]

use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, PinOps};
use arduino_hal::prelude::*;
use panic_halt as _;

const DELAI: u16 = 3600; // Une heure de fonctionnement
const DELAI_TEST: u16 = 10;
const SEUIL: u16 = 1; // Lecture au-dessus de laquelle la cafetière est considérée à ON

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
    let init = voltmètre.analog_read(&mut adc);
    ufmt::uwriteln!(
        &mut serial,
        "Seuil: {}\rLecture initiale: {}\r",
        SEUIL,
        init
    )
    .void_unwrap();

    if init > SEUIL {
        fin(&mut led); // La lecture initiale est haute : la cafetière est déjà à ON
    }
    
    let mut délai: u16 = if test.is_high() { DELAI } else { DELAI_TEST };
    let mut prec: u16 = 0;

    loop {
        let lecture = voltmètre.analog_read(&mut adc);
        if lecture != prec {
            ufmt::uwriteln!(&mut serial, "Lecture: {}\r", lecture).void_unwrap();
            prec = lecture;
        }
        if lecture > SEUIL {
            led.set_high();
            délai = délai.saturating_sub(1);
            if délai == 0 {
                crydom.set_low(); // Couper l'alimentation
                ufmt::uwriteln!(&mut serial, "Délai expiré").void_unwrap();
                fin(&mut led);
            }
        } else {
            led.set_low();
            délai = if test.is_high() { DELAI } else { DELAI_TEST };
            crydom.set_high();
        }
        arduino_hal::delay_ms(1000);
    }
}

fn fin<PB5>(led: &mut Pin<Output, PB5>) -> !
where
    PB5: PinOps,
{
    loop {
        led.toggle();
        arduino_hal::delay_ms(250);
    }
}
