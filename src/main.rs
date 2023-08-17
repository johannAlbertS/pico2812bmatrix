#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]

mod config;
use pico2812bmatrix::buffer;
use pico2812bmatrix::draw_bitmap;
// The macro for our start-up function
use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// Import the Timer for Ws2812:
use rp_pico::hal::timer::Timer;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// PIOExt for the split() method that is needed to bring
// PIO0 into useable form for Ws2812:
use rp_pico::hal::pio::PIOExt;

// Import useful traits to handle the ws2812 LEDs:
use smart_leds::{brightness, SmartLedsWrite, RGB8};

// Import the actual crate to handle the Ws2812 protocol:
use ws2812_pio::Ws2812;

mod fonts;

const led_brightness: u8 = 230;
const sleep_time: u32 = 200;

macro_rules! textbuffer_line {
    () => {
        &mut [out!(), out!(), out!(), out!(), out!(), out!(), out!()][..]
    };
}

macro_rules! r {
    ($r:expr, $g:expr, $b:expr) => {
        RGB8 {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

macro_rules! out {
    () => {
        r!(0, 0, 0)
    };
}

macro_rules! red {
    () => {
        r!(led_brightness, 0, 0)
    };
}

macro_rules! blue {
    () => {
        r!(0, 0, led_brightness)
    };
}

macro_rules! green {
    () => {
        r!(0, led_brightness, 0)
    };
    ($brightness:expr) => {
        r!(0, $brightness, 0)
    };
}

macro_rules! yellow {
    () => {
        r!(led_brightness, led_brightness, 0)
    };
}

macro_rules! white {
    () => {
        r!(led_brightness, led_brightness, led_brightness)
    };
}

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Setup a delay for the LED blink signals:
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Create a count down timer for the Ws2812 instance:
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);

    // Split the PIO state machine 0 into individual objects, so that
    // Ws2812 can use it:
    let (mut pio0, sm00, sm01, sm02, _) = pac.PIO0.split(&mut pac.RESETS);
    let (mut pio1, sm10, sm11, sm12, _) = pac.PIO1.split(&mut pac.RESETS);

    // Instanciate a Ws2812 LED strip:
    let mut ws = Ws2812::new(
        pins.gpio0.into_mode(),
        &mut pio0,
        sm00,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );
    let mut ws2 = Ws2812::new(
        pins.gpio1.into_mode(),
        &mut pio0,
        sm01,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );
    let mut ws3 = Ws2812::new(
        pins.gpio2.into_mode(),
        &mut pio0,
        sm02,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );
    let mut ws4 = Ws2812::new(
        pins.gpio3.into_mode(),
        &mut pio1,
        sm10,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );
    let mut ws5 = Ws2812::new(
        pins.gpio4.into_mode(),
        &mut pio1,
        sm11,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );
    let mut ws6 = Ws2812::new(
        pins.gpio5.into_mode(),
        &mut pio1,
        sm12,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    #[allow(non_upper_case_globals)]
    const brithness_val: u8 = 64;

    let alpaca = [
        &[
            out!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            out!(),
            out!(),
        ][..],
        &[
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            white!(),
            white!(),
            white!(),
            blue!(),
            blue!(),
            out!(),
        ][..],
        &[
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            white!(),
            white!(),
            green!(255),
            white!(),
            white!(),
            blue!(),
            blue!(),
        ][..],
        &[
            out!(),
            blue!(),
            blue!(),
            blue!(),
            white!(),
            white!(),
            white!(),
            white!(),
            white!(),
            white!(),
            blue!(),
        ][..],
        &[
            out!(),
            out!(),
            blue!(),
            blue!(),
            white!(),
            white!(),
            white!(),
            red!(),
            red!(),
            white!(),
            blue!(),
        ][..],
        &[
            out!(),
            out!(),
            blue!(),
            blue!(),
            white!(),
            white!(),
            white!(),
            white!(),
            red!(),
            white!(),
            blue!(),
        ][..],
        &[
            out!(),
            out!(),
            blue!(),
            blue!(),
            white!(),
            white!(),
            white!(),
            red!(),
            red!(),
            white!(),
            blue!(),
        ][..],
        &[
            out!(),
            blue!(),
            blue!(),
            blue!(),
            white!(),
            white!(),
            white!(),
            white!(),
            white!(),
            white!(),
            blue!(),
        ][..],
        &[
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            white!(),
            white!(),
            green!(255),
            white!(),
            white!(),
            blue!(),
            blue!(),
        ][..],
        &[
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            white!(),
            white!(),
            white!(),
            blue!(),
            blue!(),
            out!(),
        ][..],
        &[
            out!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            blue!(),
            out!(),
            out!(),
        ][..],
    ];
    let mut text_buffer = textbuffer!();
    pico2812bmatrix::fonts(&fonts::fonts);
    pico2812bmatrix::initialize_text_buffer(&mut text_buffer, &config::TEXT, red!());
    loop {
        pico2812bmatrix::draw_text_buffer(&mut text_buffer, 1);
        draw_bitmap(&alpaca, (0, 9));
        let framebuffer = buffer();
        ws.write(brightness(
            framebuffer[0]
                .iter()
                .rev()
                .chain(framebuffer[1].iter().rev())
                .copied(),
            brithness_val,
        ))
        .unwrap();
        ws2.write(brightness(
            framebuffer[2]
                .iter()
                .rev()
                .chain(framebuffer[3].iter().rev())
                .copied(),
            brithness_val,
        ))
        .unwrap();
        ws3.write(brightness(
            framebuffer[4]
                .iter()
                .rev()
                .chain(framebuffer[5].iter().rev())
                .copied(),
            brithness_val,
        ))
        .unwrap();
        ws4.write(brightness(
            framebuffer[6]
                .iter()
                .rev()
                .chain(framebuffer[7].iter().rev())
                .copied(),
            brithness_val,
        ))
        .unwrap();
        ws5.write(brightness(
            framebuffer[8]
                .iter()
                .rev()
                .chain(framebuffer[9].iter().rev())
                .copied(),
            brithness_val,
        ))
        .unwrap();
        ws6.write(brightness(
            framebuffer[10]
                .iter()
                .rev()
                .chain(framebuffer[11].iter().rev())
                .copied(),
            brithness_val,
        ))
        .unwrap();
        pico2812bmatrix::shift_text_buffer(&mut text_buffer);
        delay.delay_ms(sleep_time);
    }
}
