use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use super::bus::BusDevice;
use super::cartridge::Cartridge;

#[derive(Default, Clone)]
pub struct CtrlReg {
    pub nametable_x: bool,        // 0
    pub nametable_y: bool,        // 1
    pub increment: bool,          // 2
    pub pattern_sprite: bool,     // 3
    pub pattern_background: bool, // 4
    pub is_wide_sprite: bool,     // 5
    pub master_slave: bool,       // 6
    pub generate_nmi: bool,       // 7
}

impl CtrlReg {
    pub fn set_byte(&mut self, b: u8) {
        self.nametable_x = ((b >> 0) & 1) != 0;
        self.nametable_y = ((b >> 1) & 1) != 0;
        self.increment = ((b >> 2) & 1) != 0;
        self.pattern_sprite = ((b >> 3) & 1) != 0;
        self.pattern_background = ((b >> 4) & 1) != 0;
        self.is_wide_sprite = ((b >> 5) & 1) != 0;
        self.master_slave = ((b >> 6) & 1) != 0;
        self.generate_nmi = ((b >> 7) & 1) != 0;
    }
}

#[derive(Default, Clone)]
pub struct MaskReg {
    pub grayscale: bool,            // 0
    pub show_background_left: bool, // 1
    pub show_sprites_left: bool,    // 2
    pub show_background: bool,      // 3
    pub show_sprites: bool,         // 4
    pub emphasize_red: bool,        // 5
    pub emphasize_green: bool,      // 6
    pub emphasize_blue: bool,       // 7
}

impl MaskReg {
    pub fn set_byte(&mut self, b: u8) {
        self.grayscale = ((b >> 0) & 1) != 0;
        self.show_background_left = ((b >> 1) & 1) != 0;
        self.show_sprites_left = ((b >> 2) & 1) != 0;
        self.show_background = ((b >> 3) & 1) != 0;
        self.show_sprites = ((b >> 4) & 1) != 0;
        self.emphasize_red = ((b >> 5) & 1) != 0;
        self.emphasize_green = ((b >> 6) & 1) != 0;
        self.emphasize_blue = ((b >> 7) & 1) != 0;
    }
}

#[derive(Default, Clone)]
pub struct StatusReg {
    pub unused: u8,            // 5 bits
    pub sprite_overflow: bool, // 5
    pub sprite_zero_hit: bool, // 6
    pub vertical_blank: bool,  // 7
}

impl StatusReg {
    pub fn to_byte(&self) -> u8 {
        self.unused
            | (self.sprite_overflow as u8) << 5
            | (self.sprite_zero_hit as u8) << 6
            | (self.vertical_blank as u8) << 7
    }
}

#[derive(Default, Clone)]
pub struct LoopyAddr {
    pub coarse_x: u8, // 5 bits
    pub coarse_y: u8, // 5 bits
    pub nametable_x: bool,
    pub nametable_y: bool,
    pub fine_y: u8, // 3 bits
    pub unused: bool,
}

impl LoopyAddr {
    pub fn set_data(&mut self, data: u16) {
        self.coarse_x = (data as u8) & 0b00011111;
        self.coarse_y = ((data >> 5) as u8) & 0b00011111;
        self.nametable_x = ((data >> 10) & 1) != 0;
        self.nametable_y = ((data >> 11) & 1) != 0;
        self.fine_y = ((data >> 12) as u8) & 0b00000111;
        self.unused = ((data >> 15) & 1) != 0;
    }

    pub fn to_data(&self) -> u16 {
        ((self.coarse_x & 0b00011111) as u16) << 0
            | ((self.coarse_y & 0b00011111) as u16) << 5
            | (self.nametable_x as u16) << 10
            | (self.nametable_y as u16) << 11
            | ((self.fine_y & 0b00000111) as u16) << 12
            | (self.unused as u16) << 15
    }
}

const PALETTE: [u32; 64] = [
    0x7C7C7C, 0x0000FC, 0x0000BC, 0x4428BC, 0x940084, 0xA80020, 0xA81000, 0x881400, 0x503000,
    0x007800, 0x006800, 0x005800, 0x004058, 0x000000, 0x000000, 0x000000, 0xBCBCBC, 0x0078F8,
    0x0058F8, 0x6844FC, 0xD800CC, 0xE40058, 0xF83800, 0xE45C10, 0xAC7C00, 0x00B800, 0x00A800,
    0x00A844, 0x008888, 0x000000, 0x000000, 0x000000, 0xF8F8F8, 0x3CBCFC, 0x6888FC, 0x9878F8,
    0xF878F8, 0xF85898, 0xF87858, 0xFCA044, 0xF8B800, 0xB8F818, 0x58D854, 0x58F898, 0x00E8D8,
    0x787878, 0x000000, 0x000000, 0xFCFCFC, 0xA4E4FC, 0xB8B8F8, 0xD8B8F8, 0xF8B8F8, 0xF8A4C0,
    0xF0D0B0, 0xFCE0A8, 0xF8D878, 0xD8F878, 0xB8F8B8, 0xB8F8D8, 0x00FCFC, 0xF8D8F8, 0x000000,
    0x000000,
];

#[derive(Default, Clone, Copy)]
pub struct ObjectAttributeEntry {
    pub y: u8,
    pub id: u8,
    pub attr: u8,
    pub x: u8,
}

#[derive(Default, Clone, Copy)]
pub struct SpriteRenderState {
    pub scanline: [ObjectAttributeEntry; 8],
    count: u8,
    shifter_pattern_lo: [u8; 8],
    shifter_pattern_hi: [u8; 8],
}

#[derive(Default, Clone, Copy)]
pub struct BgRenderState {
    tile_id: u8,
    tile_attrib: u8,
    tile_lsb: u8,
    tile_msb: u8,
    shifter_pattern_lo: u16,
    shifter_pattern_hi: u16,
    shifter_attrib_lo: u16,
    shifter_attrib_hi: u16,
}

pub const SCREEN_SIZE: (usize, usize) = (256, 240);

/// Screen buffer.
pub struct Screen {
    buffer: Vec<u32>,
    pub complete: bool,
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            buffer: vec![0; SCREEN_SIZE.0 * SCREEN_SIZE.1],
            complete: true,
        }
    }
}

impl Screen {
    pub fn set_pixel(&mut self, x: usize, y: usize, c: u32) {
        self.buffer[x + y * SCREEN_SIZE.0] = c;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        self.buffer[x + y * SCREEN_SIZE.0]
    }
}

pub struct Ppu {
    pub cartridge: Rc<RefCell<Cartridge>>,
    pub screen: Screen,
    pub cycle: i16,
    pub scanline: i16,
    pub odd_frame: bool,

    pub ctrl: CtrlReg,
    pub mask: MaskReg,
    pub status: StatusReg,

    pub nmi: bool,

    pub oam_addr: u8,
    pub oam_mem: [ObjectAttributeEntry; 64],

    pub name_table: [[u8; 1024]; 2],
    pub pal_table: [u8; 32],

    pub bg_state: BgRenderState,
    pub sprite_state: SpriteRenderState,

    pub sprite_zero_hit_possible: bool,
    pub sprite_zero_being_rendered: bool,

    // Loopy:
    pub loopy_latch: bool,
    pub ppu_data_buf: u8,
    pub vram_addr: LoopyAddr,
    pub tram_addr: LoopyAddr,

    pub fine_x: u8,
}

impl BusDevice for Ppu {
    fn get_addr_range(&self) -> &Range<u16> {
        &(0x2000..0x3FFF)
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr & 0x0007 {
            0x0000 => {
                self.ctrl.set_byte(data);
                self.tram_addr.nametable_x = self.ctrl.nametable_x;
                self.tram_addr.nametable_y = self.ctrl.nametable_y;
            }
            0x0001 => {
                self.mask.set_byte(data);
            }
            0x0002 => {
                // status
            }
            0x0003 => {
                self.oam_addr = data;
            }
            0x0004 => {
                self.write_oam(self.oam_addr, data);
            }
            0x0005 => {
                // Scroll
                if self.loopy_latch == false {
                    self.fine_x = data & 0x07;
                    self.tram_addr.coarse_x = data >> 3;
                    self.loopy_latch = true;
                } else {
                    self.tram_addr.fine_y = data & 0x07;
                    self.tram_addr.coarse_y = data >> 3;
                    self.loopy_latch = false;
                }
            }
            0x0006 => {
                // PPU Address
                if self.loopy_latch == false {
                    self.tram_addr.set_data(
                        (((data as u16) & 0x3F) << 8) | (self.tram_addr.to_data() & 0x00FF),
                    );
                    self.loopy_latch = true;
                } else {
                    self.tram_addr
                        .set_data((self.tram_addr.to_data() & 0xFF00) | (data as u16));
                    self.vram_addr = self.tram_addr.clone();
                    self.loopy_latch = false;
                }
            }
            0x0007 => {
                // PPU Data

                self.ppu_write(self.vram_addr.to_data(), data);
                self.incr_vram_addr();
            }
            _ => {}
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        let mut data: u8 = 0;

        match addr & 0x0007 {
            0x0002 => {
                // Status
                data = (self.status.to_byte() & 0xE0) | (self.ppu_data_buf & 0x1F);
                self.status.vertical_blank = false;
                self.loopy_latch = false;
            }
            0x0004 => {
                // OAM Data
                let index = (self.oam_addr / 4) as usize;
                data = {
                    match self.oam_addr % 4 {
                        0 => self.oam_mem[index].y,
                        1 => self.oam_mem[index].id,
                        2 => self.oam_mem[index].attr,
                        3 => self.oam_mem[index].x,
                        _ => 0,
                    }
                };
            }
            0x0007 => {
                // PPU Data
                data = self.ppu_data_buf;
                self.ppu_data_buf = self.ppu_read(self.vram_addr.to_data());
                if self.vram_addr.to_data() >= 0x3F00 {
                    data = self.ppu_data_buf;
                }
                self.incr_vram_addr();
            }
            _ => {}
        }

        data
    }
}

impl BgRenderState {
    pub fn load_shifters(&mut self) {
        self.shifter_pattern_lo = (self.shifter_pattern_lo & 0xFF00) | (self.tile_lsb as u16);
        self.shifter_pattern_hi = (self.shifter_pattern_hi & 0xFF00) | (self.tile_msb as u16);
        self.shifter_attrib_lo = self.shifter_attrib_lo & 0xFF00;
        if self.tile_attrib & 0b01 != 0 {
            self.shifter_attrib_lo |= 0xFF;
        }
        self.shifter_attrib_hi = self.shifter_attrib_hi & 0xFF00;
        if self.tile_attrib & 0b10 != 0 {
            self.shifter_attrib_hi |= 0xFF;
        }
    }

    pub fn update_shifters(&mut self) {
        self.shifter_pattern_lo <<= 1;
        self.shifter_pattern_hi <<= 1;
        self.shifter_attrib_lo <<= 1;
        self.shifter_attrib_hi <<= 1;
    }
}

impl SpriteRenderState {
    pub fn update_shifters(&mut self) {
        for i in 0..self.count as usize {
            if self.scanline[i].x > 0 {
                self.scanline[i].x -= 1;
            } else {
                self.shifter_pattern_lo[i] <<= 1;
                self.shifter_pattern_hi[i] <<= 1;
            }
        }
    }
}

fn flipbyte(b: u8) -> u8 {
    let mut b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
    b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
    b = (b & 0xAA) >> 1 | (b & 0x55) << 1;
    b
}

impl Ppu {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        Ppu {
            cartridge: cartridge,
            screen: Screen::default(),
            cycle: 0,
            scanline: 0,
            odd_frame: false,
            ctrl: CtrlReg::default(),
            mask: MaskReg::default(),
            status: StatusReg::default(),
            nmi: false,
            oam_addr: 0,
            oam_mem: [ObjectAttributeEntry::default(); 64],
            name_table: [[0; 1024]; 2],
            pal_table: [0; 32],
            bg_state: BgRenderState::default(),
            sprite_state: SpriteRenderState::default(),
            sprite_zero_hit_possible: false,
            sprite_zero_being_rendered: false,
            loopy_latch: false,
            ppu_data_buf: 0,
            vram_addr: LoopyAddr::default(),
            tram_addr: LoopyAddr::default(),
            fine_x: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Ppu::new(self.cartridge.clone());
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) {
        let addr = addr & 0x3FFF;
        if addr < 0x2000 {
            self.cartridge.borrow_mut().ppu_write(addr, data);
        } else if addr <= 0x3EFF {
            let addr = addr & 0x0FFF;
            if self.cartridge.borrow_mut().is_vertical_mirror() {
                if addr <= 0x03FF || addr >= 0x0800 && addr <= 0x0BFF {
                    self.name_table[0][(addr & 0x03FF) as usize] = data;
                } else {
                    self.name_table[1][(addr & 0x03FF) as usize] = data;
                }
            } else {
                if addr <= 0x07FF {
                    self.name_table[0][(addr & 0x03FF) as usize] = data;
                } else {
                    self.name_table[1][(addr & 0x03FF) as usize] = data;
                }
            }
        } else if addr <= 0x3FFF {
            let mut addr = addr & 0x001F;
            addr = {
                match addr {
                    0x0010 => 0x0000,
                    0x0014 => 0x0004,
                    0x0018 => 0x0008,
                    0x001C => 0x000C,
                    _ => addr,
                }
            };
            self.pal_table[addr as usize] = data;
        }
    }

    pub fn ppu_read(&mut self, addr: u16) -> u8 {
        let mut data: u8 = 0;
        let addr = addr & 0x3FFF;

        if addr < 0x2000 {
            return self.cartridge.borrow_mut().ppu_read(addr);
        }

        if addr <= 0x3EFF {
            let addr = addr & 0x0FFF;
            if self.cartridge.borrow_mut().is_vertical_mirror() {
                if addr <= 0x03FF || addr >= 0x0800 && addr <= 0x0BFF {
                    data = self.name_table[0][(addr & 0x03FF) as usize];
                } else {
                    data = self.name_table[1][(addr & 0x03FF) as usize];
                }
            } else {
                if addr <= 0x07FF {
                    data = self.name_table[0][(addr & 0x03FF) as usize];
                } else {
                    data = self.name_table[1][(addr & 0x03FF) as usize];
                }
            }
        } else if addr <= 0x3FFF {
            let mut addr = addr & 0x001F;
            addr = {
                match addr {
                    0x0010 => 0x0000,
                    0x0014 => 0x0004,
                    0x0018 => 0x0008,
                    0x001C => 0x000C,
                    _ => addr,
                }
            };
            data = self.pal_table[addr as usize] & (if self.mask.grayscale { 0x30 } else { 0x3F });
        }

        data
    }

    pub fn write_oam(&mut self, addr: u8, data: u8) {
        let index = (addr / 4) as usize;
        match addr % 4 {
            0 => {
                self.oam_mem[index].y = data;
            }
            1 => {
                self.oam_mem[index].id = data;
            }
            2 => {
                self.oam_mem[index].attr = data;
            }
            3 => {
                self.oam_mem[index].x = data;
            }
            _ => {}
        }
    }

    fn incr_vram_addr(&mut self) {
        let mut reg = self.vram_addr.to_data();
        reg = reg.wrapping_add(if self.ctrl.increment { 32 } else { 1 });
        self.vram_addr.set_data(reg);
    }

    fn incr_scroll_x(&mut self) {
        if self.mask.show_background || self.mask.show_sprites {
            if self.vram_addr.coarse_x == 31 {
                self.vram_addr.coarse_x = 0;
                self.vram_addr.nametable_x = !self.vram_addr.nametable_x;
            } else {
                self.vram_addr.coarse_x += 1;
            }
        }
    }

    fn incr_scroll_y(&mut self) {
        if self.mask.show_background || self.mask.show_sprites {
            if self.vram_addr.fine_y < 7 {
                self.vram_addr.fine_y += 1;
            } else {
                self.vram_addr.fine_y = 0;
                if self.vram_addr.coarse_y == 29 {
                    self.vram_addr.coarse_y = 0;
                    self.vram_addr.nametable_y = !self.vram_addr.nametable_y;
                } else if self.vram_addr.coarse_y == 31 {
                    self.vram_addr.coarse_y = 0;
                } else {
                    self.vram_addr.coarse_y += 1;
                }
            }
        }
    }

    fn transfer_addr_x(&mut self) {
        if self.mask.show_background || self.mask.show_sprites {
            self.vram_addr.nametable_x = self.tram_addr.nametable_x;
            self.vram_addr.coarse_x = self.tram_addr.coarse_x;
        }
    }

    fn transfer_addr_y(&mut self) {
        if self.mask.show_background || self.mask.show_sprites {
            self.vram_addr.fine_y = self.tram_addr.fine_y;
            self.vram_addr.nametable_y = self.tram_addr.nametable_y;
            self.vram_addr.coarse_y = self.tram_addr.coarse_y;
        }
    }

    fn update_shifters(&mut self) {
        if self.mask.show_background {
            self.bg_state.update_shifters();
        }

        if self.mask.show_sprites && self.cycle >= 1 && self.cycle < 258 {
            self.sprite_state.update_shifters();
        }
    }

    pub fn clock(&mut self) {
        if self.scanline >= -1 && self.scanline < 240 {
            if self.scanline == 0
                && self.cycle == 0
                && self.odd_frame
                && (self.mask.show_background || self.mask.show_sprites)
            {
                // odd frame cycle skip
                self.cycle = 1;
            }

            if self.scanline == -1 && self.cycle == 1 {
                // New frame
                self.status.vertical_blank = false;
                self.status.sprite_overflow = false;
                self.status.sprite_zero_hit = false;
                for i in 0..8 {
                    self.sprite_state.shifter_pattern_lo[i] = 0;
                    self.sprite_state.shifter_pattern_hi[i] = 0;
                }
            }

            if (self.cycle >= 2 && self.cycle < 258) || (self.cycle >= 321 && self.cycle < 338) {
                self.update_shifters();

                match (self.cycle - 1) % 8 {
                    0 => {
                        self.bg_state.load_shifters();

                        self.bg_state.tile_id =
                            self.ppu_read(0x2000 | (self.vram_addr.to_data() & 0x0FFF));
                    }
                    2 => {
                        self.bg_state.tile_attrib = self.ppu_read(
                            0x23C0
                                | ((self.vram_addr.nametable_y as u16) << 11)
                                | ((self.vram_addr.nametable_x as u16) << 10)
                                | (((self.vram_addr.coarse_y as u16) >> 2) << 3)
                                | ((self.vram_addr.coarse_x as u16) >> 2),
                        );

                        if self.vram_addr.coarse_y & 0x02 != 0 {
                            self.bg_state.tile_attrib >>= 4;
                        }
                        if self.vram_addr.coarse_x & 0x02 != 0 {
                            self.bg_state.tile_attrib >>= 2;
                        }
                        self.bg_state.tile_attrib &= 0x03;
                    }
                    4 => {
                        self.bg_state.tile_lsb = self.ppu_read(
                            ((self.ctrl.pattern_background as u16) << 12)
                                + ((self.bg_state.tile_id as u16) << 4)
                                + ((self.vram_addr.fine_y as u16) + 0),
                        );
                    }
                    6 => {
                        self.bg_state.tile_msb = self.ppu_read(
                            ((self.ctrl.pattern_background as u16) << 12)
                                + ((self.bg_state.tile_id as u16) << 4)
                                + ((self.vram_addr.fine_y as u16) + 8),
                        );
                    }
                    7 => {
                        self.incr_scroll_x();
                    }
                    _ => {}
                }
            }

            if self.cycle == 256 {
                self.incr_scroll_y();
            }

            if self.cycle == 257 {
                self.bg_state.load_shifters();
                self.transfer_addr_x();
            }

            if self.cycle == 338 || self.cycle == 340 {
                self.bg_state.tile_id = self.ppu_read(0x2000 | (self.vram_addr.to_data() & 0x0FFF));
            }

            if self.scanline == -1 && self.cycle >= 280 && self.cycle < 305 {
                self.transfer_addr_y();
            }

            // Sprite Rendering
            if self.cycle == 257 && self.scanline >= 0 {
                self.sprite_state.scanline = [ObjectAttributeEntry::default(); 8];
                self.sprite_state.count = 0;

                for i in 0..8 {
                    self.sprite_state.shifter_pattern_lo[i] = 0;
                    self.sprite_state.shifter_pattern_hi[i] = 0;
                }

                let mut oam_index: u8 = 0;
                self.sprite_zero_hit_possible = false;

                while oam_index < 64 && self.sprite_state.count < 9 {
                    let diff = (self.scanline as i16) - (self.oam_mem[oam_index as usize].y as i16);

                    let sprite_size = if self.ctrl.is_wide_sprite { 16 } else { 8 };
                    if diff >= 0 && diff < sprite_size && self.sprite_state.count < 8 {
                        if self.sprite_state.count < 8 {
                            if oam_index == 0 {
                                self.sprite_zero_hit_possible = true;
                            }

                            self.sprite_state.scanline[self.sprite_state.count as usize] =
                                self.oam_mem[oam_index as usize];
                        }
                        self.sprite_state.count += 1;
                    }
                    oam_index += 1;
                }

                self.status.sprite_overflow = self.sprite_state.count >= 8;
            }

            if self.cycle == 340 {
                for i in 0..self.sprite_state.count as usize {
                    let mut sprite_pattern_addr_lo: u16;
                    let sprite_pattern_addr_hi: u16;

                    let scan_id = self.sprite_state.scanline[i].id as u16;
                    let scan_y = self.scanline - self.sprite_state.scanline[i].y as i16;

                    if !self.ctrl.is_wide_sprite {
                        // 8x8 sprite
                        if self.sprite_state.scanline[i].attr & 0x80 == 0 {
                            sprite_pattern_addr_lo = ((self.ctrl.pattern_sprite as u16) << 12)
                                | (scan_id << 4)
                                | (scan_y as u16);
                        } else {
                            sprite_pattern_addr_lo = ((self.ctrl.pattern_sprite as u16) << 12)
                                | (scan_id << 4)
                                | ((7 - scan_y) as u16);
                        }
                    } else {
                        // 8x16 sprite
                        sprite_pattern_addr_lo = ((scan_id & 0x01) << 12) | ((scan_id & 0xFE) << 4);

                        if self.sprite_state.scanline[i].attr & 0x80 != 0 {
                            if scan_y < 8 {
                                sprite_pattern_addr_lo |= (scan_y as u16) & 0x07;
                            } else {
                                sprite_pattern_addr_lo |= (scan_y as u16) & 0x07;
                            }
                        } else {
                            if scan_y < 8 {
                                sprite_pattern_addr_lo |= (7 - scan_y) as u16 & 0x07;
                            } else {
                                sprite_pattern_addr_lo |= (7 - scan_y) as u16 & 0x07;
                            }
                        }
                    }

                    sprite_pattern_addr_hi = sprite_pattern_addr_lo + 8;

                    let mut sprite_pattern_bits_lo = self.ppu_read(sprite_pattern_addr_lo);
                    let mut sprite_pattern_bits_hi = self.ppu_read(sprite_pattern_addr_hi);

                    if self.sprite_state.scanline[i].attr & 0x40 != 0 {
                        sprite_pattern_bits_lo = flipbyte(sprite_pattern_bits_lo);
                        sprite_pattern_bits_hi = flipbyte(sprite_pattern_bits_hi);
                    }

                    self.sprite_state.shifter_pattern_lo[i] = sprite_pattern_bits_lo;
                    self.sprite_state.shifter_pattern_hi[i] = sprite_pattern_bits_hi;
                }
            }
        }

        if self.scanline >= 241 && self.scanline < 261 {
            if self.scanline == 241 && self.cycle == 1 {
                self.status.vertical_blank = true;

                if self.ctrl.generate_nmi {
                    self.nmi = true;
                }
            }
        }

        // Compose!

        // BG
        let mut bg_pixel: u8 = 0x00;
        let mut bg_palette: u8 = 0x00;

        if self.mask.show_background {
            if self.mask.show_background_left || (self.cycle >= 9) {
                let bit_mux: u16 = 0x8000 >> self.fine_x;

                let p0_pixel: u8 = ((self.bg_state.shifter_pattern_lo & bit_mux) > 0) as u8;
                let p1_pixel: u8 = ((self.bg_state.shifter_pattern_hi & bit_mux) > 0) as u8;

                bg_pixel = (p1_pixel << 1) | p0_pixel;

                let bg_pal0: u8 = ((self.bg_state.shifter_attrib_lo & bit_mux) > 0) as u8;
                let bg_pal1: u8 = ((self.bg_state.shifter_attrib_hi & bit_mux) > 0) as u8;
                bg_palette = (bg_pal1 << 1) | bg_pal0;
            }
        }

        // FG
        let mut fg_pixel: u8 = 0x00;
        let mut fg_palette: u8 = 0x00;
        let mut fg_priority: bool = false;

        if self.mask.show_sprites {
            if self.mask.show_sprites_left || self.cycle >= 9 {
                self.sprite_zero_being_rendered = false;

                for i in 0..self.sprite_state.count as usize {
                    if self.sprite_state.scanline[i].x == 0 {
                        // Determine the pixel value...
                        let fg_pixel_lo: u8 =
                            ((self.sprite_state.shifter_pattern_lo[i] & 0x80) > 0) as u8;
                        let fg_pixel_hi: u8 =
                            ((self.sprite_state.shifter_pattern_hi[i] & 0x80) > 0) as u8;
                        fg_pixel = (fg_pixel_hi << 1) | fg_pixel_lo;

                        fg_palette = (self.sprite_state.scanline[i].attr & 0x03) + 0x04;
                        fg_priority = (self.sprite_state.scanline[i].attr & 0x20) == 0;

                        if fg_pixel != 0 {
                            if i == 0 {
                                self.sprite_zero_being_rendered = true;
                            }
                            break;
                        }
                    }
                }
            }
        }

        // Compose/Order
        let mut pixel: u8 = 0x00;
        let mut palette: u8 = 0x00;

        if bg_pixel == 0 && fg_pixel == 0 {
            pixel = 0x00;
            palette = 0x00;
        } else if bg_pixel == 0 && fg_pixel > 0 {
            pixel = fg_pixel;
            palette = fg_palette;
        } else if bg_pixel > 0 && fg_pixel == 0 {
            pixel = bg_pixel;
            palette = bg_palette;
        } else if bg_pixel > 0 && fg_pixel > 0 {
            if fg_priority {
                pixel = fg_pixel;
                palette = fg_palette;
            } else {
                pixel = bg_pixel;
                palette = bg_palette;
            }

            if self.sprite_zero_hit_possible && self.sprite_zero_being_rendered {
                if self.mask.show_background & self.mask.show_sprites {
                    if !(self.mask.show_background_left | self.mask.show_sprites_left) {
                        if self.cycle >= 9 && self.cycle < 258 {
                            self.status.sprite_zero_hit = true;
                        }
                    } else {
                        if self.cycle >= 1 && self.cycle < 258 {
                            self.status.sprite_zero_hit = true;
                        }
                    }
                }
            }
        }

        // Set pixel
        let c = self.get_color_from_pal(palette, pixel);
        if self.cycle > 0 && self.cycle <= 256 && self.scanline >= 0 && self.scanline < 240 {
            self.screen
                .set_pixel((self.cycle - 1) as usize, self.scanline as usize, c);
        }

        // advance cycles
        self.cycle += 1;

        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 261 {
                self.scanline = -1;
                self.screen.complete = true;
                self.odd_frame = !self.odd_frame;
            }
        }
    }

    pub fn get_color_from_pal(&mut self, palette: u8, pixel: u8) -> u32 {
        return PALETTE[(self.ppu_read(0x3F00 + ((palette << 2) + pixel) as u16) & 0x3F) as usize];
    }
}
