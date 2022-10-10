// PSP Resolution 480 x 272

#![no_std]
#![no_main]

// External modules
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::{ascii::FONT_6X12, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::Text;
use psp::dprintln;
use tinybmp::Bmp;

// This module does not come as standard in the psp
// crate. You must add the 'embedded-graphics' extra
// feature for it to work.
use psp::embedded_graphics::Framebuffer;

// Declares a PSP Module, giving a name and a version major and minor.
psp::module!("sample_module", 1, 1);

fn psp_main() {
    // enable the home button
    psp::enable_home_button();

    // creates a new framebuffer, which stores the colour
    // values of each pixel on the display. 
    let mut disp = Framebuffer::new();

    // Define a style and build it.
    // This still has a white fill colour. 
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb888::WHITE)
        .build();

    // Creates a black rectangle which is 160 x 80
    let black_backdrop = Rectangle::new(Point::new(0, 0), 
        Size::new(480, 272)).into_styled(style);
    
    // Draw the backdrop to the display
    // unwrap returns the err value of draw()
    black_backdrop.draw(&mut disp).unwrap();

    // Draw a sprite to the screen
    // load the bmp file as a byte string
    let bmp = Bmp::from_slice(include_bytes!("../ferris.bmp")).unwrap();
    // create a new image, defining a bmp and a positon
    let image = Image::new(&bmp, Point::zero());
    // draw the image to the display and check err value 
    image.draw(&mut disp).unwrap();

    // Draw some text to the screen
    // Define a new colour
    let text_colour = Rgb888::new(0xff, 0x07, 0x00);
    // Define the text and draw it to the display
    Text::new(
        "Hello PSP from Rust!",
        Point::new(0, 86),
        MonoTextStyle::new(&FONT_6X12, text_colour),
    )
    .draw(&mut disp)
    .unwrap();

}