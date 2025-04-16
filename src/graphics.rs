use piston_window::*;
use ::image::{ImageBuffer, Rgba};


// an 80x60 framebuffer of 8-bit tile values
pub struct FrameBuffer {
    pub width: u32, // number of tiles in the x direction
    pub height: u32, // number of tiles in the y direction
    pub tiles: Vec<u16>,
    pub buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub texture: G2dTexture,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32, mut window: PistonWindow) -> Self {
        window.set_ups(60); // Set updates per second

        let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        let texture = Texture::from_image(
            &mut window.create_texture_context(),
            &buffer,
            &TextureSettings::new(),
        ).unwrap();

        FrameBuffer {
            width,
            height,
            tiles: vec![0; ((width * height)/2) as usize],
            buffer,
            texture,
        }
    }

    pub fn set_tile_pair(&mut self, i: u32, tile_pair_value: u16) {
        // we're packing 2 tiles into 1 byte
        if i < self.tiles.len() as u32 {
            self.tiles[i as usize] = tile_pair_value;
            let id1 = (tile_pair_value & 0x8F) as u8;
            let id2 = ((tile_pair_value >> 8) & 0x8F) as u8;
            // we now get the actual tile bitmaps from a list of bitmaps
            // however, for now we will just set the tile to a solid color
            let x1 = (i % self.width) * 8;
            let x2 = ((i + 1) % self.width) * 8;
            let y = (i / self.width) * 8;
            for dy in 0..8 {
                for dx in 0..8 {
                    let px1 = (x1 + dx) as u32;
                    let px2 = (x2 + dx) as u32;
                    let py = (y + dy) as u32;

                    // set the pixel color based on the tile value
                    // for now, we'll just use a solid color
                    let color1 = [(id1%4)*64, (id1/16%4)*64, (id1/64%4)*64, 255];
                    let color2 = [(id2%4)*64, (id2/16%4)*64, (id2/64%4)*64, 255];
                    self.buffer.put_pixel(px1, py, Rgba(color1));
                    self.buffer.put_pixel(px2, py, Rgba(color2));
                }
            }
        } else {
            panic!("Tile coordinates out of bounds");
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
}


pub struct Tile {
    pub pixels: Vec<u8>, // an 8x8 tile of pixels
}


pub fn main_() {
    let (width, height) = (640, 480);
    let mut window: PistonWindow = WindowSettings::new("JPEB", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let cx = width as i32 / 2;
    let cy = height as i32 / 2;
    let radius = 100;

    // Manually draw a filled circle using the midpoint circle algorithm (or brute-force)
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                let px = (cx + x) as u32;
                let py = (cy + y) as u32;
                if px < width && py < height {
                    buffer.put_pixel(px, py, Rgba([255, 0, 0, 255])); // Red circle
                }
            }
        }
    }

    let mut texture_context = window.create_texture_context();
    let texture = Texture::from_image(
        &mut texture_context,
        &buffer,
        &TextureSettings::new(),
    ).unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _| {
            clear([0.0; 4], graphics); // black background
            image(&texture, context.transform, graphics);
        });
    }
}
