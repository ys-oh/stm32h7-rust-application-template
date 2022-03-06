

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32h7xx_hal::hal::digital::v2::OutputPin;
use stm32h7xx_hal::{pac};

use log::info;

#[entry]
fn main() -> ! {
    //hprintln!("Hello, world!").unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    
    loop {
    }
}

