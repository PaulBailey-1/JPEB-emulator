use piston_window::*;
use ::image::{ImageBuffer, Rgba};
use std::sync::{Arc, Mutex, RwLock};

use crate::memory::*;

pub struct Graphics {
    window: PistonWindow,
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    texture: G2dTexture,
    frame_buffer: Arc<RwLock<FrameBuffer>>,
    tile_map: Arc<RwLock<TileMap>>
}

impl Graphics {

    pub fn new(frame_buffer: Arc<RwLock<FrameBuffer>>, tile_map: Arc<RwLock<TileMap>>) -> Graphics {
        let mut window: PistonWindow = WindowSettings::new("JPEB", [FRAME_WIDTH, FRAME_HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap();
        window.set_max_fps(60);
        window.set_ups(60);

        let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(FRAME_WIDTH, FRAME_HEIGHT);
        let texture = Texture::from_image(
            &mut texture_context,
            &buffer,
            &TextureSettings::new(),
        ).unwrap();

        Graphics { 
            window,
            buffer,
            texture,
            frame_buffer,
            tile_map
        }
    }

    pub fn start(&mut self, finished: Arc<Mutex<bool>>, stay_open: bool) {
        while let Some(event) = self.window.next() {
            match event {
                Event::Loop(Loop::Update(_args)) => {
                    // Automatically closes window on program finish
                    if !stay_open && *finished.lock().unwrap() {
                        self.window.set_should_close(true);
                    }
                    self.update();
                }
                Event::Loop(Loop::Render(_args)) => {
                    self.window.draw_2d(&event, |context, graphics, _| {
                        clear([0.0; 4], graphics); // black background
                        image(&self.texture, context.transform, graphics);
                    });
                }
                _ => {}
            }
        }
    }

    fn update(&mut self) {
        // Updates buffer from emulated frame buffer and tile map
        let fb = self.frame_buffer.read().unwrap();
        let tile_map = self.tile_map.read().unwrap();
        for x in 0..fb.width {
            for y in 0..fb.height {
                let tile_ptr = fb.get_tile(x, y);
                let tile = &tile_map.tiles[tile_ptr as usize];
                for px in 0..TILE_SIZE {
                    for py in 0..TILE_SIZE {
                        let tile_pixel: u16 = tile.pixels[(px + py * TILE_SIZE) as usize];
                        let red = (tile_pixel & 0x000f) as u8 * 16;
                        let green = (tile_pixel & 0x00f0 >> 4) as u8 * 16;
                        let blue = (tile_pixel & 0x0f00 >> 8) as u8 * 16;
                        let pixel = Rgba([red, green, blue, 255]);
                        self.buffer.put_pixel(x * TILE_SIZE + px, y * TILE_SIZE + py, pixel);
                    }
                }
            }
        }
        // Updates texture from buffer
        self.texture = Texture::from_image(
            &mut self.window.create_texture_context(),
            &self.buffer,
            &TextureSettings::new(),
        ).unwrap();
    }
}