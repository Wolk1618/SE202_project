#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32l4xx_hal::{pac, prelude::*};
use panic_probe as _;
use defmt_rtt as _;
use tp_led_matrix::{Image, Color, matrix::Matrix};

#[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    run(cp, dp)
}

fn run(_cp: pac::CorePeripherals, dp: pac::Peripherals) -> ! {
    // Get high-level representations of hardware modules
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    // Setup the clocks at 80MHz using HSI (by default since HSE/MSI are not configured).
    // The flash wait states will be configured accordingly.
    let _clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

    //let gradient = Image::gradient(BLUE);
    //let matrix = Matrix::new(pa2, pa3, pa4, pa5, pa6, pa7, pa15, pb0, pb1, pb2, pc3, pc4, pc5, gpioa_moder, gpioa_otyper, gpiob_moder, gpiob_otyper, gpioc_moder, gpioc_otyper, clocks);

    //defmt::info!("Hello, world!");
    panic!("The program stopped");
}
