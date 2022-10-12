// PSP Resolution 480 x 272

#![no_std]
#![no_main]

use psp::{dprint, dprintln};
use psp::sys::{SceCtrlData, CtrlButtons, CtrlMode, sceRtcGetTickResolution, sceRtcGetCurrentTick};
/* PSP OS System API

The names of functions and types beginning with sce or Sce were found by reverse engineering various PSP games and OS versions.

sceXYZ: Sony API
sceKernelXYZ: Interface to the PSP OS kernel
sceCtrlXYZ: Button control API
sceDisplayXYZ: Display API
sceGeXYZ: Interface to the graphics chip (Graphics Engine)
sceUsb: USB API
sceUsbCam: USB camera
scePower: Power API
sceWlan: Wireless network API
sceRtc: Real time clock API
sceIo: File I/O API
sceAudio: Audio API
sceAtrac: Sony ATRAC3 Codec API
sceJpeg: JPEG decoding API
sceUmd: UMD Drive API
sceMpeg: MPEG codec API
sceHprm: Headphone Remote API (headphone accessory with controls)
sceGu: Graphics API (Similar to OpenGL)
sceGum: Matrix utility functions
sceMp3: MP3 decoder API
sceRegistry: PSP OS Registry API
sceOpenPSID: Console identification API (unique to every console)
sceUtility: Various utilities such as msg dialogs and savedata */

// External modules
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::{ascii::FONT_6X12, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::{Text, renderer};
use tinybmp::Bmp;

// This module does not come as standard in the psp
// crate. You must add the 'embedded-graphics' extra
// feature for it to work.
use psp::embedded_graphics::Framebuffer;

// Declares a PSP Module, giving a name and a version major and minor.
psp::module!("Simple-Game", 1, 1);

// Define width/height and graphics buffer
// const PSP_BUF_WIDTH: u32 = 512;
// const PSP_SCR_WIDTH: u32 = 480;
// const PSP_SCR_HEIGHT: u32 = 272;

mod gfx;
mod time;

fn psp_main() {
    
    // Global variables
    //let mut thing = 1;

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

    // PSP Input
    unsafe {
        psp::sys::sceCtrlSetSamplingCycle(0);
        psp::sys::sceCtrlSetSamplingMode(CtrlMode::Analog);

        let ctrl_data = &mut SceCtrlData::default();

        // loop {
        //     psp::sys::sceCtrlReadBufferPositive(ctrl_data, 1);

        //     if ctrl_data.buttons.contains(CtrlButtons::LEFT) {
        //         // move left
        //     }

        //     if ctrl_data.buttons.contains(CtrlButtons::RIGHT) {
        //         // move right
        //     }
        // }
    }

    

    // Move cube 
    unsafe {
        let mut renderer = gfx::Renderer::new();

        let mut speed = 0;
        
        // Trying simplified delta time
        let mut t = 0.0;
        let dt = 1.0 / 60.0;


        let mut cube_x_pos = 10;

        // Calc DT
        // double QuickGame_Timer_Delta(QGTimer* timer) {
        //     u64 current;
        //     sceRtcGetCurrentTick(&current);

        // 	double dt = (double)(current - timer->last) / ((double)timer->resolution);
        //     timer->total += dt;

        //     timer->last = current;

        //     return dt;
        // }

        // float lastTick = getMilliseconds();
        // float dt = 0;
        // while ( window.isOpen() ) 
        // {
        // 	//inaccurate but often close enough
        //     dt = (getMilliseconds() - lastTick);
        //     lastTick = getMilliseconds();

        // 	m_currState.input ( dt ); //dt for delta time
        // 	m_currState.update ( dt );
        // 	m_currState.draw ( dt );
        // }

        let mut last_tick: u64 = 0;
        let mut dt: u64 = 0;


        loop {

            // Clear the screen with a colour
            //renderer.clear(0xFFFFCA82);

            // Calculate delta time
            //dt = (sceRtcGetTickResolution() as u64 - last_tick) / sceRtcGetTickResolution() as u64;
            //sceRtcGetCurrentTick(&mut last_tick);


            dprintln!("{}",sceRtcGetTickResolution());

            // Move cube
            unsafe {
                psp::sys::sceCtrlSetSamplingCycle(0);
                psp::sys::sceCtrlSetSamplingMode(CtrlMode::Analog);
        
                let ctrl_data = &mut SceCtrlData::default();
        
                psp::sys::sceCtrlReadBufferPositive(ctrl_data, 1);
                
                // Move right
                if ctrl_data.buttons.contains(CtrlButtons::RIGHT) {
                    cube_x_pos = cube_x_pos + dt * 10;
                }

                // Move left
                if ctrl_data.buttons.contains(CtrlButtons::LEFT) {
                    cube_x_pos = cube_x_pos - dt * 272;
                }
    
            }


            //renderer.draw_rect(cube_x_pos as usize, 10, 30, 30, 0x0FF00FFFF);


            // Draw next frame
            //renderer.swap_buffers();
            //psp::sys::sceDisplayWaitVblankStart();
        }
    }

}