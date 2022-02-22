#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m_rt::entry;
use stm32f4::stm32f401; // (1)

#[entry]
fn main() -> ! {
    let dp = stm32f401::Peripherals::take().unwrap();   // (2)
    dp.RCC.ahb1enr.modify(|_, w| w.gpioaen().enabled());    // (3)
    let gpioa = &dp.GPIOA;                                  // (4)
    gpioa.moder.modify(|_, w| w.moder5().output());         // (5)
    
    loop {
        if gpioa.idr.read().idr6().is_low() {    // (6)GPIOA6を読む
            gpioa.odr.modify(|_, w| w.odr5().high());       // (7)
        }
        else {
            gpioa.odr.modify(|_, w| w.odr5().low());        // (8)
        }
    }
}
