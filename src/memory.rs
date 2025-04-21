use std::collections::VecDeque;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub const STACK_START : usize = 0xA000;

pub const FRAME_WIDTH: u32 = 1024;
pub const FRAME_HEIGHT: u32 = 512;
pub const TILE_SIZE: u32 = 8;
const TILES_NUM: u32 = 128;
const TILE_DATA_SIZE: u32 = TILE_SIZE * TILE_SIZE;
pub const SPRITE_SIZE: u32 = 32;
const SPRITES_NUM: u32 = 8;
const SPRITE_DATA_SIZE: u32 = SPRITE_SIZE * SPRITE_SIZE;

const TILE_MAP_START : usize = 0xC000;
const TILE_MAP_SIZE : usize = 0x2000;
const FRAME_BUFFER_START : usize = 0xE000;
const FRAME_BUFFER_SIZE : usize = 0x1000;
const IO_BUFFER_START : usize = 0xFFFF;
const V_SCROLL_START : usize = 0xFFFE;
const H_SCROLL_START : usize = 0xFFFD;
const SCALE_REGISTER_START : usize = 0xFFFC; // each pixel is repeated 2^n times
const SPRITE_MAP_START : usize = 0xA000;
const SPRITE_MAP_SIZE : usize = 0x1000;
const SPRITE_REGISTERS_START : usize = 0xFFE0;  // every consecutive pair of words correspond to 
const SPIRTE_REGISTERS_SIZE : usize = 0x10;     // the y and x coordinates, respectively of a sprite

pub struct Memory {
  ram: Vec<u16>,   
  frame_buffer: Arc<RwLock<FrameBuffer>>,
  tile_map: Arc<RwLock<TileMap>>, 
  io_buffer: Arc<RwLock<VecDeque<u16>>>,
  vscroll_register: Arc<RwLock<u16>>,
  hscroll_register: Arc<RwLock<u16>>,
  scale_register: Arc<RwLock<u16>>,
  sprite_map: Arc<RwLock<SpriteMap>>,
}

// an 80x60 framebuffer of 8-bit tile values
pub struct FrameBuffer {
    pub width: u32, // number of tiles in the x direction
    pub height: u32, // number of tiles in the y direction
    tile_ptrs: Vec<u16>,
}

pub struct TileMap {
    pub tiles: Vec<Tile>
}

#[derive(Clone)]
pub struct Tile {
    pub pixels: Vec<u16>, // an 8x8 tile of pixels
}

pub struct SpriteMap {
    pub sprites: Vec<Sprite>,
}

#[derive(Clone)]
pub struct Sprite {
    pub x: u16,
    pub y: u16,
    pub pixels: Vec<u16>, // a 32x32 tile of pixels
}

impl Memory {

    pub fn new(ram_init: Vec<u16>, datapath: &str) -> Memory {
        // Fill ram to size of address space
        let mut ram = ram_init;
        ram.resize(1 << 16, 0);

        Memory {
            ram,
            frame_buffer: Arc::new(RwLock::new(FrameBuffer::new(FRAME_WIDTH, FRAME_HEIGHT))),
            tile_map: Arc::new(RwLock::new(TileMap::load(&format!("{datapath}/tilemap.bmp")))),
            io_buffer: Arc::new(RwLock::new(VecDeque::new())),
            vscroll_register: Arc::new(RwLock::new(0)),
            hscroll_register: Arc::new(RwLock::new(0)),
            scale_register: Arc::new(RwLock::new(0)),
            sprite_map: Arc::new(RwLock::new(SpriteMap::load(&format!("{datapath}/spritemap.bmp"))))
        }
    }

    pub fn get_frame_buffer(&self) -> Arc<RwLock<FrameBuffer>> { return Arc::clone(&self.frame_buffer)}
    pub fn get_tile_map(&self) -> Arc<RwLock<TileMap>> { return Arc::clone(&self.tile_map)}
    pub fn get_io_buffer(&self) -> Arc<RwLock<VecDeque<u16>>> { return Arc::clone(&self.io_buffer) }
    pub fn get_vscroll_register(&self) -> Arc<RwLock<u16>> { return Arc::clone(&self.vscroll_register) }
    pub fn get_hscroll_register(&self) -> Arc<RwLock<u16>> { return Arc::clone(&self.hscroll_register) }
    pub fn get_scale_register(&self) -> Arc<RwLock<u16>> { return Arc::clone(&self.scale_register) }
    pub fn get_sprite_map(&self) -> Arc<RwLock<SpriteMap>> { return Arc::clone(&self.sprite_map) }

    pub fn read(&mut self, addr: usize) -> u16 {
        if addr >= TILE_MAP_START && addr < TILE_MAP_START + TILE_MAP_SIZE {
            return self.tile_map.read().unwrap().get_tile_word((addr - TILE_MAP_START) as u32);
        }
        if addr >= FRAME_BUFFER_START && addr < FRAME_BUFFER_START + FRAME_BUFFER_SIZE {
            return self.frame_buffer.read().unwrap().get_tile_pair((addr - FRAME_BUFFER_START) as u32);
        }
        if addr == IO_BUFFER_START {
            return self.io_buffer.write().unwrap().pop_front().unwrap_or(0);
        }
        if addr >= SPRITE_MAP_START && addr < SPRITE_MAP_START + SPRITE_MAP_SIZE {
            return self.sprite_map.read().unwrap().get_sprite_word((addr - SPRITE_MAP_START) as u32);
        }
        if addr >= SPRITE_REGISTERS_START && addr < SPRITE_REGISTERS_START + SPIRTE_REGISTERS_SIZE {
            return self.sprite_map.read().unwrap().get_sprite_reg((addr - SPRITE_REGISTERS_START) as u32);
        }
        if addr == V_SCROLL_START {
            return *self.vscroll_register.read().unwrap();
        }
        if addr == H_SCROLL_START {
            return *self.hscroll_register.read().unwrap();
        }
        if addr == SCALE_REGISTER_START {
            return *self.scale_register.read().unwrap();
        }
        return self.ram[addr];
    }

    pub fn write(&mut self, addr: usize, data: u16) {
        if addr >= TILE_MAP_START && addr < TILE_MAP_START + TILE_MAP_SIZE {
            self.tile_map.write().unwrap().set_tile_word((addr - TILE_MAP_START) as u32, data);
        }
        if addr >= FRAME_BUFFER_START && addr < FRAME_BUFFER_START + FRAME_BUFFER_SIZE {
            self.frame_buffer.write().unwrap().set_tile_pair((addr - FRAME_BUFFER_START) as u32, data);
        }
        if addr == IO_BUFFER_START {
            panic!("attempting to write to read input port (address {})", IO_BUFFER_START);
        }
        if addr == V_SCROLL_START {
            *self.vscroll_register.write().unwrap() = data;
        }
        if addr == H_SCROLL_START {
            *self.hscroll_register.write().unwrap() = data;
        }
        if addr == SCALE_REGISTER_START {
            *self.scale_register.write().unwrap() = data;
        }
        if addr >= SPRITE_MAP_START && addr < SPRITE_MAP_START + SPRITE_MAP_SIZE {
            self.sprite_map.write().unwrap().set_sprite_word((addr - SPRITE_MAP_START) as u32, data);
        }
        if addr >= SPRITE_REGISTERS_START && addr < SPRITE_REGISTERS_START + SPIRTE_REGISTERS_SIZE {
            self.sprite_map.write().unwrap().set_sprite_reg((addr - SPRITE_REGISTERS_START) as u32, data);
        }
        if addr == 0 {
            println!("Writing to address 0x0000: 0x{:04X}", data);
        }
        self.ram[addr] = data;
    }
}

impl FrameBuffer {
    pub fn new(frame_width: u32, frame_height: u32) -> Self {
        let width = frame_width / TILE_SIZE;
        let height = frame_height / TILE_SIZE;
        FrameBuffer {
            width,
            height,
            tile_ptrs: vec![0; (width * height / 2) as usize],
        }
    }

    pub fn set_tile_pair(&mut self, i: u32, tile_pair_value: u16) {
        // we're packing 2 tile_ptrs into 1 word
        if i < self.tile_ptrs.len() as u32 {
            self.tile_ptrs[i as usize] = tile_pair_value;
        } else {
            panic!("Tile coordinates out of bounds: {}", i);
        }
    }

    pub fn get_tile_pair(&self, i: u32) -> u16 {
        // we're packing 2 tile_ptrs into 1 word
        if i < self.tile_ptrs.len() as u32 {
            return self.tile_ptrs[i as usize];
        } else {
            panic!("Tile coordinates out of bounds");
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> u8 {
        if x < self.width && y < self.height {
            let idx: usize = (x + y * self.width) as usize;
            if idx % 2 == 0 {
                return (self.tile_ptrs[idx / 2] & 0x00ff) as u8;
            } else {
                return (self.tile_ptrs[idx / 2] >> 8) as u8;
            }
        } else {
            panic!("Tile coordinates out of bounds");
        }
    }
}

impl Tile {
    pub fn black() -> Tile {
        Tile {
            pixels: vec![0; TILE_DATA_SIZE as usize]
        }
    }
    pub fn white() -> Tile {
        Tile {
            pixels: vec![0xffff; TILE_DATA_SIZE as usize]
        }
    }
}

impl TileMap {
    pub fn new(size: usize) -> TileMap {
        let tiles = vec![Tile::black(); size];
        TileMap { 
            tiles
        }
    }

    pub fn load(filename: &str) -> TileMap {
        let img = bmp::open(filename).expect(&format!("Failed to open tilemap {}", filename));
        if (img.get_width() * img.get_height()) / (TILE_SIZE * TILE_SIZE) != TILES_NUM {
            panic!("Loaded tilemap size mismatch");
        }

        let mut tiles: Vec<Tile> = vec![];
        for y in 0..(img.get_height() / TILE_SIZE) {
            for x in 0..(img.get_width() / TILE_SIZE) {
                let mut pixels: Vec<u16> = vec![];
                for py in 0..TILE_SIZE {
                    for px in 0..TILE_SIZE {
                        let p = img.get_pixel(x * TILE_SIZE + px, y * TILE_SIZE + py);
                        let mut color: u16 = 0;
                        color = color | ((p.r >> 4) as u16);
                        color = color | (((p.g >> 4) as u16) << 4);
                        color = color | (((p.b >> 4) as u16) << 8);
                        pixels.push(color);
                    }
                }
                tiles.push(Tile{pixels});
            }
        }
        let map = TileMap{tiles};
        let mut path = PathBuf::from(filename);
        path.set_extension(".hex");
        map.save_hex_map(path.to_str().unwrap()).expect("Failed to save hex tile map");
        return map;
    }

    pub fn save_bin_map(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to open bin output");
        let mut data: Vec<u8> = vec![];
        for tile in &self.tiles {
            for py in 0..TILE_SIZE {
                for px in 0..TILE_SIZE {
                    let p = tile.pixels[(py * TILE_SIZE + px) as usize];
                    data.push((p & 0x00ff) as u8);
                    data.push(((p & 0xff00) >> 8) as u8);
                }
            }
        }
        file.write_all(data.as_slice()).expect("Failed to write to bin");
    }

    pub fn save_hex_map(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(filename)?;
        file.write(b"@0\n")?;
        for tile in &self.tiles {
            for py in 0..TILE_SIZE {
                for px in 0..TILE_SIZE {
                    let p = tile.pixels[(py * TILE_SIZE + px) as usize];
                    write!(file, "{:04X} ", p)?;
                }
            }
            file.write(b"\n")?;
        }
        Ok(())
    }

    pub fn get_tile_word(&self, addr: u32) -> u16 {
        return self.tiles[(addr / TILE_DATA_SIZE) as usize].pixels[(addr % TILE_DATA_SIZE) as usize];
    }

    pub fn set_tile_word(&mut self, addr: u32, data: u16) {
        self.tiles[(addr / TILE_DATA_SIZE) as usize].pixels[(addr % TILE_DATA_SIZE) as usize] = data;
    }
}

impl Sprite {
    pub fn invisible() -> Sprite {
        Sprite {
            x: 0xFFFF,
            y: 0xFFFF,
            pixels: vec![0xFFFF; SPRITE_DATA_SIZE as usize],
        }
    }
}

impl SpriteMap {
    pub fn new(size: usize) -> SpriteMap {
        let sprites = vec![Sprite::invisible(); size];
        SpriteMap { 
            sprites
        }
    }

    pub fn load(filename: &str) -> SpriteMap {
        let img = bmp::open(filename).expect(&format!("Failed to open spritemap {}", filename));
        if (img.get_width() * img.get_height()) / (SPRITE_SIZE * SPRITE_SIZE) < SPRITES_NUM {
            panic!("Loaded spritemap size mismatch");
        }

        let mut sprites: Vec<Sprite> = vec![];
        for y in 0..(img.get_height() / SPRITE_SIZE) {
            for x in 0..(img.get_width() / SPRITE_SIZE) {
                let mut pixels: Vec<u16> = vec![];
                for py in 0..SPRITE_SIZE {
                    for px in 0..SPRITE_SIZE {
                        let p = img.get_pixel(x * SPRITE_SIZE + px, y * SPRITE_SIZE + py);
                        if p == bmp::Pixel::new(0xff, 0x00, 0xff) {
                            // transparent pixel
                            pixels.push(0xFFFF);
                            continue;
                        }

                        let mut color: u16 = 0;
                        color = color | ((p.r >> 4) as u16);
                        color = color | (((p.g >> 4) as u16) << 4);
                        color = color | (((p.b >> 4) as u16) << 8);
                        pixels.push(color);
                    }
                }
                sprites.push(Sprite{x: 0xFFFF, y: 0xFFFF, pixels});
            }
        }
        // only store the first 8 sprites
        sprites.truncate(SPRITES_NUM as usize);
        let map = SpriteMap{sprites};
        let mut path = PathBuf::from(filename);
        path.set_extension(".hex");
        map.save_hex_map(path.to_str().unwrap()).expect("Failed to save hex sprite map");
        return map;
    }

    pub fn save_bin_map(&self, filename: &str) {
        let mut file = File::create(filename).expect("Failed to open sprite output");
        let mut data: Vec<u8> = vec![];
        for sprite in &self.sprites {
            for py in 0..SPRITE_SIZE {
                for px in 0..SPRITE_SIZE {
                    let p = sprite.pixels[(py * SPRITE_SIZE + px) as usize];
                    data.push((p & 0x00ff) as u8);
                    data.push(((p & 0xff00) >> 8) as u8);
                }
            }
        }
        file.write_all(data.as_slice()).expect("Failed to write to bin");
    }

    pub fn save_hex_map(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(filename)?;
        file.write(b"@0\n")?;
        for sprite in &self.sprites {
            for py in 0..SPRITE_SIZE {
                for px in 0..SPRITE_SIZE {
                    let p = sprite.pixels[(py * SPRITE_SIZE + px) as usize];
                    write!(file, "{:04X}\n", p)?;
                }
            }
            file.write(b"\n")?;
        }
        Ok(())
    }

    // this will get a single corrsponding pixel
    pub fn get_sprite_word(&self, addr: u32) -> u16 {
        return self.sprites[(addr / SPRITE_DATA_SIZE) as usize].pixels[(addr % SPRITE_DATA_SIZE) as usize];
    }

    pub fn set_sprite_word(&mut self, addr: u32, data: u16) {
        self.sprites[(addr / SPRITE_DATA_SIZE) as usize].pixels[(addr % SPRITE_DATA_SIZE) as usize] = data;
    }

    // returns the either y or x coordinate of the sprite corresponding to the addr/2, addr%2
    pub fn get_sprite_reg(&self, addr: u32) -> u16 {
        let sprite = &self.sprites[(addr / 2) as usize];
        if addr % 2 == 0 {
            return sprite.x;
        } else {
            return sprite.y;
        }
    }

    // sets the either y or x coordinate of the sprite corresponding to the addr/2, addr%2
    pub fn set_sprite_reg(&mut self, addr: u32, data: u16) {
        let sprite = &mut self.sprites[(addr / 2) as usize];
        if addr % 2 == 0 {
            sprite.x = data;
        } else {
            sprite.y = data;
        }
    }
}