// Created on 04 March 2024
// Just a joke project to learn Rust for Arduino
// When a photoresistor detects the Google dinosaur, the servo clicks space to make it jump
#![no_std]
#![no_main] 

use panic_halt as _;
use arduino_hal::simple_pwm::*;

const MIN_DUTY: u8 = 10;
const MAX_DUTY: u8 = 14 ;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);

    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale1024);

    let mut pwm_led = pins.d9.into_output().into_pwm(&timer1);
    pwm_led.enable();

    loop {

        let value: u16 = a0.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "{} ", value).unwrap();

        if value < 550 
        {
            pwm_led.set_duty(MIN_DUTY);
            arduino_hal::delay_ms(300);
            pwm_led.set_duty(MAX_DUTY);
        }
        else 
        {
            pwm_led.set_duty(MAX_DUTY);
        }
    }
}
