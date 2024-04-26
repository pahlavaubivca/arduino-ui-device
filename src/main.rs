#![no_std]
#![no_main]


use panic_halt as _;

mod lcd;

use arduino_hal::prelude::{_embedded_hal_blocking_spi_Write, _unwrap_infallible_UnwrapInfallible};
use arduino_hal::{hal, Spi};
use arduino_hal::hal::delay::Delay;
use arduino_hal::spi::{DataOrder, SerialClockRate, Settings};
use embedded_graphics::geometry::Point;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi;
 use crate::lcd::lcd::{Orientation, ST7735};
use embedded_graphics::{
    mono_font::{
         MonoTextStyleBuilder,
    },
    prelude::*,
    text::Text,
};
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::mono_font::ascii::{FONT_6X12, FONT_8X13};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics_core::pixelcolor::Rgb565;

/*todo
   1) spi_master - find where find and how to use
   2)
   */

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // let pinss = arduino_hal::pins!(dp.USART0);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);


    // println!("Hello, world!");
   ufmt::uwriteln!(&mut serial, "Start program! {} \r",1).unwrap_infallible();


    // if let Some(loc) = info.location() {
    //     ufmt::uwriteln!(
    //         &mut serial,
    //         "  At {}:{}:{}\r",
    //         loc.file(),
    //         loc.line(),
    //         loc.column(),
    //     )
    //         .unwrap_infallible();
    // }
    let mut sclk_pin = pins.d13.into_output();
    let mosi_pin = pins.d11.into_output();
    let miso_pin = pins.d12.into_pull_up_input();
    let cs_pin = pins.d10.into_output();
    let dc = pins.d3.into_output();
    let settings = Settings {
        data_order: DataOrder::MostSignificantFirst,
        clock: SerialClockRate::OscfOver4,
        mode: spi::Mode {
            polarity: spi::Polarity::IdleLow,
            phase: spi::Phase::CaptureOnFirstTransition,
        },
    };
    let rst = pins.d2.into_output();
    let _spi: Spi = arduino_hal::Spi::new(
        dp.SPI,
        sclk_pin,
        mosi_pin,
        miso_pin,
        cs_pin,
        settings,
    ).0;

    let mut disp = ST7735::new(_spi, dc,
                               Some(rst),
                               true, false, 128, 128);
    // let mut delay = Delay::new();
    let mut delay = Delay::<hal::clock::MHz12>::new();
    disp.init(&mut delay).unwrap();
    disp.set_offset(2, 1);
    disp.clear(Rgb565::BLACK).unwrap();
    _=Text::new(
        "Hello 6x12!",
        Point::new(15, 45),
        MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE/*BinaryColor::Off*/),
    )
        .draw(&mut disp);
    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    // let mut led = pins.d13.into_output();
    let mut count = 0;
    loop {
        count += 1;
        disp.clear(Rgb565::BLACK).unwrap();
        // let count_as_str = count.to_string();
        let buffer = itoa(count);
        let s = core::str::from_utf8(&buffer).unwrap().trim();
        let result = concat_strs_simple("Hello 6x12! ", s);
        let message = core::str::from_utf8(&result).unwrap();
        // let message = sprinft!("Hello 6x12! {}", count);
        _=Text::new(

            message,
            Point::new(15, 65),
            MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE/*BinaryColor::Off*/),
        )
            .draw(&mut disp);

        // sclk_pin.toggle();
        arduino_hal::delay_ms(1000);
    }
}


fn itoa(mut num: i32) -> [u8; 12] {  // Buffer size includes space for negative sign and null terminator
    let mut buffer = [b' '; 12]; // Initialize buffer with spaces
    let mut i = 10; // Start filling buffer from the end

    if num == 0 {
        buffer[i] = b'0'; // Handle zero explicitly
        return buffer;
    }

    let is_negative = num < 0;
    if is_negative {
        num = -num; // Make number positive for processing
    }

    while num > 0 {
        buffer[i] = (b'0' + (num % 10) as u8);
        num /= 10;
        i -= 1;
    }

    if is_negative {
        i -= 1;
        buffer[i] = b'-';
    }

    // Shift the buffer to the left to remove leading spaces
    let mut result = [b' '; 12];
    let start = i;
    let mut j = 0;
    while i < 12 {
        result[j] = buffer[i];
        i += 1;
        j += 1;
    }

    result
}

fn concat_strs_simple(a: &str, b: &str) -> [u8; 32] {
    let mut buffer = [0u8; 32]; // Buffer size needs to be sufficient
    let bytes_a = a.as_bytes();
    let bytes_b = b.as_bytes();

    buffer[..bytes_a.len()].copy_from_slice(bytes_a);
    buffer[bytes_a.len()..bytes_a.len() + bytes_b.len()].copy_from_slice(bytes_b);

    buffer
}