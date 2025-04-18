use std::collections::VecDeque;

use std::fs::File;
use std::io::Write;
use std::sync::{Arc, RwLock};

pub const STACK_START : usize = 0xc000;

pub const FRAME_WIDTH: u32 = 640;
pub const FRAME_HEIGHT: u32 = 480;
pub const TILE_SIZE: u32 = 8;
const TILES_NUM: u32 = 128;
const TILE_DATA_SIZE: u32 = TILE_SIZE * TILE_SIZE;

const TILE_MAP_START : usize = 0xC000;
const TILE_MAP_SIZE : usize = 0x2000;
const FRAME_BUFFER_START : usize = 0xe000;
const FRAME_BUFFER_SIZE : usize = 0x1000;
const IO_BUFFER_START : usize = 0xFFFF;
const V_SCROLL_START : usize = 0xFFFE;
const H_SCROLL_START : usize = 0xFFFC;

pub struct Memory {
  ram: Vec<u16>,   
  frame_buffer: Arc<RwLock<FrameBuffer>>,
  tile_map: Arc<RwLock<TileMap>>, 
  io_buffer: Arc<RwLock<VecDeque<u16>>>,
  vscroll_register: Arc<RwLock<u16>>,
  hscroll_register: Arc<RwLock<u16>>,
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

impl Memory {

    pub fn new(ram_init: Vec<u16>) -> Memory {
        // Fill ram to size of address space
        let mut ram = ram_init;
        ram.resize(1 << 16, 0);

        Memory {
            ram,
            frame_buffer: Arc::new(RwLock::new(FrameBuffer::new(FRAME_WIDTH, FRAME_HEIGHT))),
            tile_map: Arc::new(RwLock::new(TileMap::load("tilemap.bmp"))),
            io_buffer: Arc::new(RwLock::new(VecDeque::new())),
            vscroll_register: Arc::new(RwLock::new(0)),
            hscroll_register: Arc::new(RwLock::new(0)),
        }
    }

    pub fn get_frame_buffer(&self) -> Arc<RwLock<FrameBuffer>> { return Arc::clone(&self.frame_buffer)}
    pub fn get_tile_map(&self) -> Arc<RwLock<TileMap>> { return Arc::clone(&self.tile_map)}
    pub fn get_io_buffer(&self) -> Arc<RwLock<VecDeque<u16>>> { return Arc::clone(&self.io_buffer) }
    pub fn get_vscroll_register(&self) -> Arc<RwLock<u16>> { return Arc::clone(&self.vscroll_register) }
    pub fn get_hscroll_register(&self) -> Arc<RwLock<u16>> { return Arc::clone(&self.hscroll_register) }

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
        let mut tiles = vec![Tile::black(); size];
        tiles[0] = Tile::white();
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
        map.save_bin_map();
        return map;
    }

    pub fn save_bin_map(&self) {
        let mut file = File::create("tilemap.bin").expect("Failed to open bin output");
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

    pub fn get_tile_word(&self, addr: u32) -> u16 {
        return self.tiles[(addr / TILE_DATA_SIZE) as usize].pixels[(addr % TILE_DATA_SIZE) as usize];
    }

    pub fn set_tile_word(&mut self, addr: u32, data: u16) {
        self.tiles[(addr / TILE_DATA_SIZE) as usize].pixels[(addr % TILE_DATA_SIZE) as usize] = data;
    }
}