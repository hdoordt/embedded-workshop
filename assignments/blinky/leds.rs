use nrf52840_hal::{
    gpio::{Disconnected, Level, Output, Pin, PushPull},
    prelude::OutputPin,
};

use crate::delay_millis;

pub struct Leds {
    led_1: Pin<Output<PushPull>>,
    led_2: Pin<Output<PushPull>>,
    led_3: Pin<Output<PushPull>>,
    led_4: Pin<Output<PushPull>>,
}

impl Leds {
    pub fn new(
        led_1: Pin<Disconnected>,
        led_2: Pin<Disconnected>,
        led_3: Pin<Disconnected>,
        led_4: Pin<Disconnected>,
    ) -> Leds {
        Self {
            led_1: led_1.into_push_pull_output(Level::High),
            led_2: led_2.into_push_pull_output(Level::High),
            led_3: led_3.into_push_pull_output(Level::High),
            led_4: led_4.into_push_pull_output(Level::High),
        }
    }

    pub fn animate_higher(&mut self) {
        defmt::info!("higher!");
        for _ in 0..3 {
            self.led_1.set_low();
            delay_millis(50);
            self.led_1.set_high();
            self.led_2.set_low();
            delay_millis(50);
            self.led_2.set_high();
            self.led_3.set_low();
            delay_millis(50);
            self.led_3.set_high();
            self.led_4.set_low();
            delay_millis(50);
            self.led_4.set_high();
            delay_millis(100);
        }
    }

    pub fn animate_lower(&mut self) {
        defmt::info!("lower!");
        for _ in 0..3 {
            self.led_4.set_low();
            delay_millis(50);
            self.led_4.set_high();
            self.led_3.set_low();
            delay_millis(50);
            self.led_3.set_high();
            self.led_2.set_low();
            delay_millis(50);
            self.led_2.set_high();
            self.led_1.set_low();
            delay_millis(50);
            self.led_1.set_high();
            delay_millis(100);
        }
    }

    pub fn animate_win(&mut self) {
        defmt::info!("win!");
        for _ in 0..5 {
            delay_millis(50);
            self.led_1.set_low();
            self.led_2.set_low();
            self.led_3.set_low();
            self.led_4.set_low();
            delay_millis(50);
            self.led_1.set_high();
            self.led_2.set_high();
            self.led_3.set_high();
            self.led_4.set_high();
        }
    }
}
