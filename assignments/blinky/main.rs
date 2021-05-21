#![no_std]
#![no_main]

use hal::{
    gpio::{
        p0::{Parts, P0_11, P0_12},
        Input, Level, Output, Pin, PullDown, PullUp, PushPull,
    },
    pac,
};
use nrf52840_hal as hal;

// Contains all kinds of nice extension traits
use hal::prelude::*;
use pac::Peripherals;

use core::panic::PanicInfo;
use cortex_m_rt::entry;

// Contains definitions needed for defmt,
// so we can print messages.
// Also contains a panic handler,
// so we don't need to add it here anymore
use assignments::{self as _, exit};

use crate::leds::Leds;

mod leds;

#[entry]
fn start() -> ! {
    // Get a handle to te Peripherals
    let p = Peripherals::take().unwrap();

    let port0 = Parts::new(p.P0);

    let mut leds = Leds::new(
        port0.p0_13.degrade(),
        port0.p0_14.degrade(),
        port0.p0_15.degrade(),
        port0.p0_16.degrade(),
    );

    // Initialize LED pin as output
    // Initialize button pun as input

    defmt::info!("Hello, world");

    let btn_inc = port0.p0_11.into_pullup_input().degrade();
    let btn_acc = port0.p0_12.into_pullup_input().degrade();

    let mut count: u32 = 0;

    let mut rng = hal::rng::Rng::new(p.RNG);
    let mut value = rng.random_u8() & 0x0F;
    loop {
        let input = read_value(&btn_inc, &btn_acc);
        defmt::info!("input:{}, value:{}", input, value);
        match input {
            i if value > i => leds.animate_higher(),
            i if value < i => leds.animate_lower(),
            _ => {
                leds.animate_win();
                value = rng.random_u8() & 0x0F;
            }
        }
    }
}

pub fn delay_millis(ms: u32) {
    cortex_m::asm::delay(ms * 64000)
}

fn read_value(btn_inc: &Pin<Input<PullUp>>, btn_acc: &Pin<Input<PullUp>>) -> u8 {
    let mut input: u8 = 0;
    let mut inc_pressed = false;
    while btn_acc.is_high().unwrap() {
        if btn_inc.is_low().unwrap() {
            if !inc_pressed {
                input += 1
            }
            inc_pressed = true;
        } else {
            inc_pressed = false;
        }
    }
    while btn_acc.is_low().unwrap() {}

    input
}
