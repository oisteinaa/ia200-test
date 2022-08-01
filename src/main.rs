#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;
// use cortex_m::Peripherals;
use stm32f4xx_hal::{
    pac::Peripherals, 
    gpio::GpioExt, 
    // prelude::_stm32f4xx_hal_timer_SysCounterExt,
    prelude::*, 
    rcc::RccExt
};


#[entry]
fn main() -> ! {
    // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    let p = Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    
    let gpioe = p.GPIOE.split();
    let mut yellow_ledpin = gpioe.pe7.into_push_pull_output();
    let mut green_ledpin = gpioe.pe8.into_push_pull_output();
    let mut red_ledpin = gpioe.pe9.into_push_pull_output();
    yellow_ledpin.set_low();
    green_ledpin.set_low();
    red_ledpin.set_low();

    
    let rcc = p.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(16384.kHz())
        .sysclk(168.MHz())
        .pclk1(24.MHz())
        .i2s_clk(86.MHz())
        .require_pll48clk()
        .freeze();

    let mut delay = cp.SYST.delay(&clocks);

    loop {
        yellow_ledpin.set_high();
        delay.delay_ms(30_u32);
        green_ledpin.set_high();
        delay.delay_ms(30_u32);
        red_ledpin.set_high();
        delay.delay_ms(500_u32);
        yellow_ledpin.set_low();
        delay.delay_ms(30_u32);
        green_ledpin.set_low();
        delay.delay_ms(30_u32);
        red_ledpin.set_low();
        delay.delay_ms(500_u32);

    
    }
}
