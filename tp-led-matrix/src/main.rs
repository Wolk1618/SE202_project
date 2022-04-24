#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use stm32l4xx_hal::{pac, prelude::*};
use stm32l4xx_hal::serial::{Config, Event, Rx, Serial};
use panic_probe as _;
use defmt_rtt as _;
use tp_led_matrix::{Image, matrix::Matrix};
use dwt_systick_monotonic::DwtSystick;
use dwt_systick_monotonic::ExtU32;
use crate::pac::USART1;

#[rtic::app(device = pac, dispatchers = [USART2, USART3])]
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

        // Getting representation of usefull GPIOs
        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

        // Initialising led matrix
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

        // Getting representation of transmission and reception pins for the serial port.
        let tx = gpiob.pb6.into_alternate::<7>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
        let rx = gpiob.pb7.into_alternate::<7>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

        // Initialising and configuring the serial port.
        let config = Config::default().baudrate(38_400.bps());
        let mut serial = Serial::usart1(dp.USART1, (tx, rx), config, clocks, &mut rcc.apb2);
        serial.listen(Event::Rxne);
        let usart1_rx = serial.split().1;

        // Initialising local and shared image variables.
        let image = Image::default();
        let next_image = Image::default();

        // First launch of the display task.
        display::spawn(mono.now()).unwrap();

        //rotate_image::spawn(mono.now(), 0).unwrap();

        // Return the resources and the monotonic timer
        (Shared {image}, Local {matrix, usart1_rx, next_image}, init::Monotonics(mono))
    }

    #[shared]
    struct Shared {
        image : Image,
    }

    #[local]
    struct Local {
        matrix: Matrix,
        usart1_rx : Rx<USART1>,
        next_image : Image,
    }

    #[idle(local = [])]
    fn idle(_cx: idle::Context) -> ! {
        loop {} // infinite loop because this idle task isn't usefull for this program.
    }

    #[task(local = [matrix, next_line: usize = 0], shared = [image], priority = 2)]
    fn display(mut cx: display::Context, at: Instant) {
        // Display line next_line (cx.local.next_line) of
        // the image (cx.local.image) on the matrix (cx.local.matrix).
        // All those are mutable references.
        cx.shared.image.lock( |image| {
            cx.local.matrix.send_row(*cx.local.next_line, image.row(*cx.local.next_line));
        } );
        // Increment next_line up to 7 and wraparound to 0
        *cx.local.next_line = (*cx.local.next_line + 1) % 8;

        // Computing the amount of time so that a frame is displayed at a rate of 60 fps.
        let time : Instant = at + 1.secs() / (60 * 8);
        display::spawn_at(time, time).unwrap();
    }

    #[task(binds = USART1, local = [usart1_rx, next_image, next_pos: usize = 0], shared = [image])]
    fn receive_byte(mut cx: receive_byte::Context) {
        let next_image: &mut Image = cx.local.next_image;
        let next_pos: &mut usize = cx.local.next_pos;
        if let Ok(b) = cx.local.usart1_rx.read() {
            // Handle the incoming byte according to the SE203 protocol
            // and update next_image
            // Do not forget that next_image.as_mut() might be handy here!

            if b == 0xff { // b==0xff anounces the beginning of a new image.
                *next_pos = 0;
            } else {
                let mutimage = next_image.as_mut();
                // The next image is changed pixel by pixel (and even component by component within a pixel).
                // It is possible to use next_pos to change the right byte of the image since
                // we chose to represent images as it is in the memory by adding #[repr(C)] attribute for Color
                // and #[repr(transparent)] for Image.
                mutimage[*next_pos as usize] = b;
                *next_pos += 1;
            }

            // If the received image is complete, make it available to
            // the display task.
            if *next_pos == 8 * 8 * 3 {
                cx.shared.image.lock(|image| {
                    // Replace the image content by the new one, for example
                    // by swapping them, and reset next_pos
                    core::mem::swap(image, next_image);
                    *next_pos = 0;
                });
            }
        }
    }
 

    /*#[task(shared = [image], priority = 1)]
    fn rotate_image(mut cx: rotate_image::Context, at: Instant, color_index: usize) {
        cx.shared.image.lock( |image| {
            match color_index {
                0 => *image = Image::gradient(Color::RED),
                1 => *image = Image::gradient(Color::GREEN),
                2 => *image = Image::gradient(Color::BLUE),
                _ => panic!("Wrong color value"),
            }
        } );
        rotate_image::spawn_after(1.secs(), at, (color_index + 1) % 3).unwrap();
    }*/

}

