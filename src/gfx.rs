use psp::sys::{DisplayMode, DisplayPixelFormat, DisplaySetBufSync};

pub struct Renderer {
    draw_buffer: *mut u32,
    disp_buffer: *mut u32,
}

impl Renderer {
    pub unsafe fn new() -> Self {
        let disp_buffer = psp::sys::sceGeEdramGetAddr() as *mut u32;
        let draw_buffer = psp::sys::sceGeEdramGetAddr().add(512 * 272 * 4) as *mut u32;

        psp::sys::sceDisplaySetMode(DisplayMode::Lcd, 480, 272);
        psp::sys::sceDisplaySetFrameBuf(
            disp_buffer as *const u8, 
            512, 
            DisplayPixelFormat::Psm8888, 
            DisplaySetBufSync::NextFrame
        );

        Self {
            draw_buffer,
            disp_buffer
        }
    }

    pub fn clear(&self, colour: u32) {
        unsafe {
            for i in 0..512 * 272 {
                *self.draw_buffer.add(i as usize) = colour;
            }
        }
    }

    pub fn swap_buffers(&mut self) {
        core::mem::swap(&mut self.disp_buffer, &mut self.draw_buffer);

        unsafe {
            psp::sys::sceKernelDcacheWritebackAll();
            psp::sys::sceDisplaySetFrameBuf(
                self.disp_buffer as *const u8, 
                512, 
                DisplayPixelFormat::Psm8888, 
                DisplaySetBufSync::NextFrame
            );
        }
    }

    pub fn draw_rect(&self, x: usize, y: usize, w: usize, h: usize, colour: u32) {
        for y1 in 0..h  {
            for x1 in 0..w {
                if let Some(ptr) = self.calculate_offset(x + x1, y + y1) {
                    unsafe {
                        *ptr = colour;
                    }
                }
            }
        }
    }

    #[inline]
    fn calculate_offset(&self, x: usize, y: usize ) -> Option<*mut u32> {
        unsafe {
            if x <= 480 && y <= 272 {
                Some(self.draw_buffer.add(x + y * 512))
            }
            else {
                None
            }
        }
    }
}