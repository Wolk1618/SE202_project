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
    let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

    let gradient = Image::gradient(Color::BLUE);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

    let mut matrix = Matrix::new(
        gpioa.pa2,
        gpioa.pa3,
        gpioa.pa4,
        gpioa.pa5,
        gpioa.pa6,
        gpioa.pa7,
        gpioa.pa15,
        gpiob.pb0,
        gpiob.pb1,
        gpiob.pb2,
        gpioc.pc3,
        gpioc.pc4,
        gpioc.pc5,
        &mut gpioa.moder,
        &mut gpioa.otyper,
        &mut gpiob.moder,
        &mut gpiob.otyper,
        &mut gpioc.moder,
        &mut gpioc.otyper,
        clocks);

    for i in 0..8 {
        defmt::trace!(
            "Ligne {} : {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{}",
            i,
            gradient[(i, 0)].r, gradient[(i, 0)].g, gradient[(i, 0)].b,
            gradient[(i, 1)].r, gradient[(i, 1)].g, gradient[(i, 1)].b,
            gradient[(i, 2)].r, gradient[(i, 2)].g, gradient[(i, 2)].b,
            gradient[(i, 3)].r, gradient[(i, 3)].g, gradient[(i, 3)].b,
            gradient[(i, 4)].r, gradient[(i, 4)].g, gradient[(i, 4)].b,
            gradient[(i, 5)].r, gradient[(i, 5)].g, gradient[(i, 5)].b,
            gradient[(i, 6)].r, gradient[(i, 6)].g, gradient[(i, 6)].b,
            gradient[(i, 7)].r, gradient[(i, 7)].g, gradient[(i, 7)].b
        );
    }

    loop {
        matrix.display_image(&gradient);
    }

    //defmt::info!("Hello, world!");
    //panic!("The program stopped");
}
