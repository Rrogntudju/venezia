#![no_std]
#![no_main]

use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, PinOps};
use panic_halt as _;

const DELAI: u16 = 3600; // Une heure de fonctionnement
const DELAI_TEST: u16 = 10;
const SEUIL: u16 = 30; // Seuil de lecture au-dessus duquel la cafetière est considérée à ON

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut crydom = pins.d12.into_output_high(); // Brancher l'alimentation
    let test = pins.d8.into_pull_up_input();
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let voltmètre = pins.a0.into_analog_input(&mut adc);
    let mut led = pins.d13.into_output();
    #[cfg(debug_assertions)]
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    arduino_hal::delay_ms(250); // Délai pour lecture initiale maximale
    let init = voltmètre.analog_read(&mut adc);
    #[cfg(debug_assertions)]
    {
        ufmt::uwriteln!(&mut serial, "Seuil: {}\r", SEUIL).unwrap();
        ufmt::uwriteln!(&mut serial, "Lecture initiale: {}\r", init).unwrap();
    }

    if init > SEUIL {
        #[cfg(debug_assertions)]
        ufmt::uwriteln!(&mut serial, "Lecture initiale > SEUIL").unwrap();

        crydom.set_low(); // Couper l'alimentation
        fin(&mut led); // La lecture initiale est haute : la cafetière est déjà à ON
    }

    let mut délai: u16 = if test.is_high() { DELAI } else { DELAI_TEST };
    #[cfg(debug_assertions)]
    ufmt::uwriteln!(&mut serial, "Délai: {}\r", délai).unwrap();

    loop {
        let lecture = voltmètre.analog_read(&mut adc);
        #[cfg(debug_assertions)]
        ufmt::uwriteln!(&mut serial, "Lecture: {}\r", lecture).unwrap();

        if lecture > SEUIL {
            led.set_high();
            délai = délai.saturating_sub(1);
            if délai == 0 {
                #[cfg(debug_assertions)]
                ufmt::uwriteln!(&mut serial, "Délai expiré").unwrap();

                crydom.set_low(); // Couper l'alimentation
                fin(&mut led);
            }
        } else {
            led.set_low();
            délai = if test.is_high() { DELAI } else { DELAI_TEST };
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
