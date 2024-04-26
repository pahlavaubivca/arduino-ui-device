#![no_std]
#![no_main]

mod lcd;

use arduino_hal::prelude::_embedded_hal_blocking_spi_Write;
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

    let mut sclk_pin = pins.d13.into_output();
    let mosi_pin = pins.d11.into_output();
    let miso_pin = pins.d12.into_pull_up_input();
    let cs_pin = pins.d10.into_output();
    let dc = pins.d1.into_output();
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

    // _=_spi.write(&[]).map_err(|_| ());
        // _spi.1.set_high().unwrap();
    // let command = lcd::instruction::Instruction::CASET;
    // _ = _spi.write(&[command as u8]).map_err(|_| ());
    //
    // let mut disp = ST7735::new(_spi, dc,
    //                            Some(rst),
    //                            false, true, 160, 80);
    // let mut delay = Delay::new();
    // let mut delay = Delay::<hal::clock::MHz16>::new();
    // // let mut delay = DelayNs(500u32);//::new(core.SYST, &mut clocks);
    // // let mut delay = arduino_hal::delay_ms(5000).;
    // disp.init(&mut delay).unwrap();
    // disp.set_orientation(&Orientation::Landscape).unwrap();
    // disp.set_offset(1, 26);
    // disp.clear(Rgb565::BLACK).unwrap();
    // let core = CorePeripherals::take().unwrap();
    // let mut clocks = GenericClockController::with_external_32kosc(
    //     peripherals.GCLK,
    //     &mut peripherals.MCLK,
    //     &mut peripherals.OSC32KCTRL,
    //     &mut peripherals.OSCCTRL,
    //     &mut peripherals.NVMCTRL,
    // );
    // let mut clocks = arduino_hal::hal::clock::Clock GenericClockController::with_external_32kosc(
    //     dp..GCLK,
    //     &mut peripherals.MCLK,
    //     &mut peripherals.OSC32KCTRL,
    //     &mut peripherals.OSCCTRL,
    //     &mut peripherals.NVMCTRL,
    // );

    // let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("./assets/ferris.raw"), 86);
    // let image  = Image::new(&image_raw, Point::new(34, 8));
    // image.draw(&mut disp).unwrap();
    //
    // let normal = MonoTextStyleBuilder::new()
    //     .font(&FONT_8X13)
    //     .text_color(Rgb565::WHITE)
    //     .build();
    // _=Text::new(
    //     "Hello 6x12!",
    //     Point::new(15, 45),
    //     // normal
    //     MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE/*BinaryColor::Off*/),
    // )
    //     .draw(&mut disp);

    // let output_settings = OutputSettingsBuilder::new().scale(2).build();
    // Window::new("Fonts", &output_settings).show_static(&display);
    // let err_example = embedded_hal::digital::ErrorType::PinAlreadyInUse;
    // let dc: OutputPin<embedded_hal::digital::ErrorType::PinAlreadyInUse> = pins.d1.into_output();
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

    loop {
        // sclk_pin.toggle();
        arduino_hal::delay_ms(1000);
    }
}
