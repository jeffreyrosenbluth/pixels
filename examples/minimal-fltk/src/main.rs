#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{app, enums::Event, prelude::*, window::Window};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;
const CIRCLE_RADIUS: i16 = 64;

/// Representation of the application state. In this example, a circle will bounce around the screen.
struct World {
    circle_x: i16,
    circle_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() -> Result<(), Error> {
    #[cfg(debug_assertions)]
    env_logger::init();

    let app = app::App::default();
    let mut win = Window::default()
        .with_size(WIDTH as i32, HEIGHT as i32)
        .with_label("Hello Pixels");
    win.end();
    win.show();

    let mut pixels = {
        let pixel_width = win.pixel_w() as u32;
        let pixel_height = win.pixel_h() as u32;
        let surface_texture = SurfaceTexture::new(pixel_width, pixel_height, &win);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new();

    while app.wait() {
        // Handle window events
        if app::event() == Event::Resize {
            let pixel_width = win.pixel_w() as u32;
            let pixel_height = win.pixel_h() as u32;
            pixels.resize_surface(pixel_width, pixel_height);
        }

        // Update internal state
        world.update();

        // Draw the current frame
        world.draw(pixels.get_frame_mut());
        if pixels
            .render()
            .map_err(|e| error!("pixels.render() failed: {}", e))
            .is_err()
        {
            app.quit();
        }

        app::flush();
        app::awake();
    }

    Ok(())
}

impl World {
    /// Create a new `World` instance that can draw a moving circle.
    fn new() -> Self {
        Self {
            circle_x: 300,
            circle_y: 200,
            velocity_x: 5,
            velocity_y: 5,
        }
    }

    /// Update the `World` internal state; bounce the circle around the screen.
    fn update(&mut self) {
        if self.circle_x - CIRCLE_RADIUS <= 0 || self.circle_x + CIRCLE_RADIUS > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.circle_y - CIRCLE_RADIUS <= 0 || self.circle_y + CIRCLE_RADIUS > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.circle_x += self.velocity_x;
        self.circle_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            let d = {
                let xd = x as i32 - self.circle_x as i32;
                let yd = y as i32 - self.circle_y as i32;
                ((xd.pow(2) + yd.pow(2)) as f64).sqrt().powi(2)
            };
            let inside_the_circle = d < (CIRCLE_RADIUS as f64).powi(2);

            let rgba = if inside_the_circle {
                [0xac, 0x00, 0xe6, 0xff]
            } else {
                [0x26, 0x00, 0x33, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
