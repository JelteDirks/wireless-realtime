const VGA_BUFFER_ADDR: u32 = 0xb8000;

use crate::util::delay;

// NOTE: https://www.fountainware.com/EXPL/vga_color_palettes.htm

#[allow(dead_code)]
#[derive(Default, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum VGAColorCode {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    White = 0x07,
    Gray = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0A,
    #[default]
    LightCyan = 0x0B,
    LightRed = 0x0C,
    LightMagenta = 0x0D,
    Yellow = 0x0E,
    BrightWhite = 0x0F,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct VGACharacter {
    char_code: u8,
    color_code: VGAColorCode,
}

impl From<u8> for VGACharacter {
    fn from(val: u8) -> Self {
        VGACharacter {
            char_code: match val {
                0x20..0x7e => val,
                _ => 0xfe,
            },
            color_code: VGAColorCode::default(),
        }
    }
}

pub struct VGAWriter {
    base: &'static mut [VGACharacter],
    cols: usize,
    rows: usize,
    row_cursor: usize,
    col_cursor: usize,
}

impl VGAWriter {
    pub fn new() -> Self {
        let cols: usize = 80;
        let rows: usize = 25;
        let length: usize = cols * rows;
        let start = VGA_BUFFER_ADDR as *mut VGACharacter;
        let buf = core::ptr::slice_from_raw_parts_mut(start, length);

        VGAWriter {
            base: unsafe { &mut *(buf) },
            cols,
            rows,
            row_cursor: 0,
            col_cursor: 0,
        }
    }

    fn move_lines_up(&mut self) {
        let n = self.rows * self.cols;
        for i in 0..n - self.cols {
            self.base[i] = self.base[i + self.cols];
        }

        for i in self.cols * (self.rows - 1)..(self.cols * self.rows) {
            self.base[i] = b' '.into();
        }
    }

    pub fn new_line(&mut self) {
        self.row_cursor += 1;
        self.col_cursor = 0;

        if self.row_cursor == self.rows {
            self.move_lines_up();
            self.row_cursor -= 1;
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                // NOTE: could maybe be optimized away?
                self.base[self.offset()] = byte.into();
                self.advance_cursor();
            }
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for &b in bytes.iter() {
            self.write_byte(b);
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for b in s.bytes() {
            self.write_byte(b);
        }
    }

    fn offset(&self) -> usize {
        self.row_cursor * self.cols + self.col_cursor
    }

    fn advance_cursor(&mut self) {
        if self.col_cursor == self.cols - 1 {
            self.new_line();
        } else {
            self.col_cursor += 1;
        }
    }
}
