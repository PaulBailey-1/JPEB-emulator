use piston_window::*;
use ::image::{ImageBuffer, Rgba};

// the width and height of tiles in pixels
pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = 8;
// the width and height of the framebuffer in tiles
pub const FRAMEBUFFER_WIDTH: usize = 80;
pub const FRAMEBUFFER_HEIGHT: usize = 60;

// an 8x8 tile of 16-bit RGBA pixel values
#[derive(Clone)]
pub struct Tile {
    pub pixels: Vec<u16>,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            pixels: vec![0; (TILE_WIDTH * TILE_HEIGHT) as usize],
        }
    }

    pub fn set_pixel(&mut self, i: usize, color: u16) {
        if i < self.pixels.len() {
            self.pixels[i] = color;
        } else {
            panic!("Pixel coordinates out of bounds");
        }
    }
    pub fn get_pixel(&self, i: usize) -> u16 {
        if i < self.pixels.len() {
            self.pixels[i]
        } else {
            panic!("Pixel coordinates out of bounds");
        }
    }
}

// the 128 different tiles
pub struct TileMap {
    pub tiles: Vec<Tile>,
}

impl TileMap {
    pub fn new() -> Self {
        TileMap {
            tiles: vec![Tile::new(); 128],
        }
    }

    pub fn set_tile(&mut self, i: usize, tile: Tile) {
        if i < self.tiles.len() {
            self.tiles[i] = tile;
        } else {
            panic!("Tile coordinates out of bounds");
        }
    }

    pub fn get_tile(&self, i: usize) -> &Tile {
        if i < self.tiles.len() {
            &self.tiles[i]
        } else {
            panic!("Tile coordinates out of bounds");
        }
    }

    pub fn get_pixel(&self, i: usize) -> u16 {
        let tile_index = i / (TILE_WIDTH * TILE_HEIGHT) as usize;
        let pixel_index = i % (TILE_WIDTH * TILE_HEIGHT) as usize;
        if tile_index < self.tiles.len() {
            self.tiles[tile_index].get_pixel(pixel_index)
        } else {
            panic!("Tile coordinates out of bounds");
        }
    }

    pub fn set_pixel(&mut self, i: usize, color: u16) {
        let tile_index = i / (TILE_WIDTH * TILE_HEIGHT) as usize;
        let pixel_index = i % (TILE_WIDTH * TILE_HEIGHT) as usize;
        if tile_index < self.tiles.len() {
            self.tiles[tile_index].set_pixel(pixel_index, color);
        } else {
            panic!("Tile coordinates out of bounds");
        }
    }
}


// an 80x60 framebuffer of 8-bit tile values
pub struct FrameBuffer {
    pub tiles: Vec<u16>,
    pub buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub texture: G2dTexture,
    pub tilemap: TileMap,
}

impl FrameBuffer {
    pub fn new(mut texture_context: &mut G2dTextureContext) -> Self {
        let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(
            (FRAMEBUFFER_WIDTH * TILE_WIDTH) as u32,
            (FRAMEBUFFER_HEIGHT * TILE_HEIGHT) as u32,
        );

        let texture = Texture::from_image(
            &mut texture_context,
            &buffer,
            &TextureSettings::new(),
        ).unwrap();

        FrameBuffer {
            tiles: vec![0; ((FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT)/2) as usize],
            buffer,
            texture,
            tilemap: TileMap::new(),
        }
    }

    pub fn set_tile_pair(&mut self, i: usize, tile_pair_value: u16) {
        // we're packing 2 tiles into 1 byte
        if i < self.tiles.len() {
            self.tiles[i as usize] = tile_pair_value;
            let id1 = (tile_pair_value & 0x8F) as u8;
            let id2 = ((tile_pair_value >> 8) & 0x8F) as u8;
            // we now get the actual tile bitmaps from a list of bitmaps
            // however, for now we will just set the tile to a solid color
            let x1 = (i % FRAMEBUFFER_WIDTH) * 8;
            let x2 = ((i + 1) % FRAMEBUFFER_WIDTH) * 8;
            let y = (i / FRAMEBUFFER_WIDTH) * 8;
            for dy in 0..8 {
                for dx in 0..8 {
                    let px1 = (x1 + dx) as u32;
                    let px2 = (x2 + dx) as u32;
                    let py = (y + dy) as u32;

                    // set the pixel color based on the tile value
                    // for now, we'll just use a solid color
                    let color1 = [(id1%4)*64, (id1/16%4)*64, (id1/64%4)*64, 255];
                    let color2 = [(id2%4)*64, (id2/16%4)*64, (id2/64%4)*64, 255];
                    // let color1 = [255, 0, 0, 255]; // red
                    // let color2 = [255, 0, 0, 255]; // red
                    self.buffer.put_pixel(px1, py, Rgba(color1));
                    self.buffer.put_pixel(px2, py, Rgba(color2));
                }
            }
        } else {
            // unexpected index: print an error message and the index
            panic!("Tile index out of bounds: {}", i);
        }
    }

    pub fn get_tile_pair(&self, i: u32) -> u16 {
        // we're packing 2 tiles into 1 byte
        if i < self.tiles.len() as u32 {
            self.tiles[i as usize]
        } else {
            panic!("Tile coordinates out of bounds");
        }
    }

    pub fn update_texture(&mut self, texture_context: &mut G2dTextureContext) {
        self.texture = Texture::from_image(
            texture_context,
            &self.buffer,
            &TextureSettings::new(),
        ).unwrap();
    }
}



pub fn main_() {
    let window_dimension = [
        (TILE_WIDTH * FRAMEBUFFER_WIDTH) as u32,
        (TILE_HEIGHT * FRAMEBUFFER_HEIGHT) as u32,
    ];
    let mut window: PistonWindow = WindowSettings::new("JPEB", window_dimension)
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(60); // Set updates per second

    let mut texture_context = window.create_texture_context();
    let mut buffer: FrameBuffer = FrameBuffer::new(&mut texture_context);
    buffer.tilemap.set_pixel(0, 128);

    let cx = FRAMEBUFFER_WIDTH / 2;
    let cy = FRAMEBUFFER_HEIGHT / 2;
    let radius = 20 as i32;

    // Manually draw a filled circle using the midpoint circle algorithm (or brute-force)
    for y in -radius/2..=radius/2 {
        for x in -radius/2..=radius/2 {
            if x * x + y * y <= radius/2 * radius/2 {
                let expected_px = (cx as i32/2 + x) as usize;
                let expected_py = (cy as i32/2 + y) as usize;
                let expected_index = expected_py * FRAMEBUFFER_WIDTH/2 + expected_px;
                buffer.set_tile_pair(expected_index, 0xfff0);
            }
        }
    }

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _| {
            clear([0.0; 4], graphics); // black background
            buffer.update_texture(&mut texture_context); // Update the texture with the buffer
            image(&buffer.texture, context.transform, graphics);
        });
    }
}
