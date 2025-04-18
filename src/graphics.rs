use piston_window::*;
use ::image::{ImageBuffer, Rgba};
use std::{collections::VecDeque, sync::{Arc, Mutex, RwLock}};

use crate::memory::*;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub struct Graphics {
    window: PistonWindow,
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    texture: G2dTexture,
    frame_buffer: Arc<RwLock<FrameBuffer>>,
    tile_map: Arc<RwLock<TileMap>>,
    io_buffer: Arc<RwLock<VecDeque<u16>>>,
    vscroll_register: Arc<RwLock<u16>>,
    hscroll_register: Arc<RwLock<u16>>,
    scale_register: Arc<RwLock<u16>>,
    sprite_map: Arc<RwLock<SpriteMap>>,
}

impl Graphics {

    pub fn new(
        frame_buffer: Arc<RwLock<FrameBuffer>>, 
        tile_map: Arc<RwLock<TileMap>>, 
        io_buffer: Arc<RwLock<VecDeque<u16>>>, 
        vscroll_register: Arc<RwLock<u16>>,
        hscroll_register: Arc<RwLock<u16>>,
        sprite_map: Arc<RwLock<SpriteMap>>,
        scale_register: Arc<RwLock<u16>>,
    ) -> Graphics {
        let mut window: PistonWindow = WindowSettings::new("JPEB", [SCREEN_WIDTH, SCREEN_HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap();
        window.set_max_fps(60);
        window.set_ups(60);

        let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(FRAME_WIDTH, FRAME_HEIGHT);
        let texture = Texture::from_image(
            &mut window.create_texture_context(),
            &buffer,
            &TextureSettings::new(),
        ).unwrap();

        Graphics { 
            window,
            buffer,
            texture,
            frame_buffer,
            tile_map,
            io_buffer,
            vscroll_register,
            hscroll_register,
            sprite_map,
            scale_register,
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
                Event::Input(Input::Button(ButtonArgs { 
                    button: Button::Keyboard(key), 
                    state, .. }), _) => {
                    match state {
                        ButtonState::Press => {
                            self.io_buffer.write().unwrap().push_back(key as u16);
                            // println!("Key pressed: {:?}", key);
                            // Handle key press here
                        }
                        ButtonState::Release => {
                            // println!("Key released: {:?}", key);
                            // Handle key release here
                        }
                    }
                }
                _ => {}
            }
        }
    }


    fn update(&mut self) {
        // Updates buffer from emulated frame buffer and tile map
        // draw the tiles of the frame buffer
        let fb = self.frame_buffer.read().unwrap();
        let tile_map = self.tile_map.read().unwrap();
        let scale = 1 << (*self.scale_register.read().unwrap() as u32);
        for x in 0..fb.width {
            for y in 0..fb.height {
                let tile_ptr = fb.get_tile(x, y);
                let tile = &tile_map.tiles[tile_ptr as usize];
                for px in 0..TILE_SIZE {
                    for py in 0..TILE_SIZE {
                        let tile_pixel: u16 = tile.pixels[(px + py * TILE_SIZE) as usize];
                        let red = (tile_pixel & 0x000f) as u8 * 16;
                        let green = ((tile_pixel & 0x00f0) >> 4) as u8 * 16;
                        let blue = ((tile_pixel & 0x0f00) >> 8) as u8 * 16;
                        let pixel = Rgba([red, green, blue, 255]);
                        
                        // positions in the logical screen
                        let scroll_x = *self.hscroll_register.read().unwrap() as i32;
                        let scroll_y = *self.vscroll_register.read().unwrap() as i32;
                        let raw_x: i32 = (x * TILE_SIZE) as i32 + px as i32 + scroll_x;
                        let raw_y: i32 = (y * TILE_SIZE) as i32 + py as i32 + scroll_y;
                        let final_x: u32 = (raw_x + FRAME_WIDTH as i32) as u32 % FRAME_WIDTH;
                        let final_y: u32 = (raw_y + FRAME_HEIGHT as i32) as u32 % FRAME_HEIGHT;

                        // print the pixel rgba in the physical screen
                        for i in 0..scale {
                            for j in 0..scale {
                                let screen_x: u32 = final_x * scale + i;
                                let screen_y: u32 = final_y * scale + j;

                                if screen_x < SCREEN_WIDTH && screen_y < SCREEN_HEIGHT {
                                    self.buffer.put_pixel(screen_x, screen_y, pixel);
                                }
                            }
                        }
                    }
                }
            }
        }

        // draw the sprites of the sprite map
        let sprite_map = self.sprite_map.read().unwrap();
        for sprite in &sprite_map.sprites {
            for px in 0..SPRITE_SIZE {
                for py in 0..SPRITE_SIZE {
                    let tile_pixel: u16 = sprite.pixels[(px + py * SPRITE_SIZE) as usize];
                    let red = (tile_pixel & 0x000f) as u8 * 16;
                    let green = ((tile_pixel & 0x00f0) >> 4) as u8 * 16;
                    let blue = ((tile_pixel & 0x0f00) >> 8) as u8 * 16;
                    let transparent = (tile_pixel & 0xf000) == 0xf000;
                    if transparent {
                        continue;
                    }

                    let pixel = Rgba([red, green, blue, 255]);
                    let final_x: u32 = sprite.x as u32 + px;
                    let final_y: u32 = sprite.y as u32 + py;

                    // print the pixel rgba in the physical screen
                    for i in 0..scale {
                        for j in 0..scale {
                            let screen_x: u32 = final_x * scale + i;
                            let screen_y: u32 = final_y * scale + j;

                            if screen_x < SCREEN_WIDTH && screen_y < SCREEN_HEIGHT {
                                self.buffer.put_pixel(screen_x, screen_y, pixel);
                            }
                        }
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
