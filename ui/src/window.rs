use std::sync::Arc;

use lazy8_core::CHIP8;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::Window,
};

#[derive(Default)]
pub(crate) struct App {
    pub(crate) chip8: CHIP8,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = match event_loop.create_window(Window::default_attributes()) {
            Ok(it) => it,
            Err(e) => panic!("failed to create 'winit' window. {}", e),
        };
        window.set_visible(true);
        let window = Arc::new(window);
        let window_ptr: &'static Window = Box::leak(Box::new(window.clone()));
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window_ptr);
        let pixels = match Pixels::new(64, 32, surface_texture) {
            Ok(it) => it,
            Err(e) => panic!("failed to create a 'pixels' instance. \n{}", e),
        };
        self.window = Some(window);
        self.pixels = Some(pixels);
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                log::trace!("Received window close request");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.chip8.step();
                self.render(event_loop);

                match self.window.as_ref() {
                    Some(it) => it.request_redraw(),
                    None => log::error!(
                        "Failed to handle redraw request on a window that does not exist"
                    ),
                }
            }
            _ => (),
        }
    }
}

impl App {
    fn render(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let is_on = self.chip8.display[i];
                let color = if is_on { 0xFF } else { 0x00 };
                pixel[0] = color; // R
                pixel[1] = color; // G
                pixel[2] = color; // B
                pixel[3] = 0xFF; // A (Always 255 for opaque)
            }
            if let Err(e) = pixels.render() {
                log::error!("Render error: {}", e);
                event_loop.exit();
            }
        }
    }
}
