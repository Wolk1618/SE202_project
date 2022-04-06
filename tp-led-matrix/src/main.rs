#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use stm32l4xx_hal::{pac, prelude::*};
use panic_probe as _;
use defmt_rtt as _;
use tp_led_matrix::{Image, Color, matrix::Matrix};
use dwt_systick_monotonic::DwtSystick;
use dwt_systick_monotonic::ExtU32;

#[rtic::app(device = pac, dispatchers = [USART2])]
mod app {

    use super::*;
    #[monotonic(binds = SysTick, default = true)]
    type MyMonotonic = DwtSystick<80_000_000>;
    type Instant = <MyMonotonic as rtic::Monotonic>::Instant;


    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("defmt correctly initialized");

        let mut cp = cx.core;
        let dp = cx.device;

        let mut mono = DwtSystick::new(&mut cp.DCB, cp.DWT, cp.SYST, 80_000_000);

        // Get high-level representations of hardware modules
        let mut rcc = dp.RCC.constrain();
        let mut flash = dp.FLASH.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
    
        // Setup the clocks at 80MHz using HSI (by default since HSE/MSI are not configured).
        // The flash wait states will be configured accordingly.
        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

        let matrix = Matrix::new(
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
            clocks
        );

        let image = Image::gradient(Color::BLUE);

        display::spawn(mono.now()).unwrap();

        // Return the resources and the monotonic timer
        (Shared {}, Local { matrix, image }, init::Monotonics(mono))
    }

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        matrix: Matrix,
        image : Image,
    }

    #[idle(local = [])]
    fn idle(_cx: idle::Context) -> ! {

        /*for i in 0..8 {
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
        }*/

        //let mut i = 0;
        loop {
            /*if i == 10_000 {
                defmt::info!("top");
                i = 0;
            } else {
                i += 1;
            }*/
        }
    }

    #[task(local = [matrix, image, next_line: usize = 0])]
    fn display(cx: display::Context, at: Instant) {
        // Display line next_line (cx.local.next_line) of
        // the image (cx.local.image) on the matrix (cx.local.matrix).
        // All those are mutable references.
        cx.local.matrix.send_row(*cx.local.next_line, cx.local.image.row(*cx.local.next_line));
        // Increment next_line up to 7 and wraparound to 0
        if *cx.local.next_line == 7 {
            *cx.local.next_line = 0;
        } else {
            *cx.local.next_line += 1;
        }

        let time : Instant = at + 1.secs() / (60 * 8);
        display::spawn_at(time, time).unwrap();
    }

}

