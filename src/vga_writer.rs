const VGA_BUFFER_ADDR: u32 = 0xb8000;

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

#[repr(C)]
pub struct VGACharacter {
    char_code: u8,
    color_code: VGAColorCode,
}

impl VGACharacter {
    fn new(byte: u8, color: VGAColorCode) -> Self {
        VGACharacter {
            char_code: byte,
            color_code: color,
        }
    }
}

impl From<u8> for VGACharacter {
    fn from(val: u8) -> Self {
        // TODO: some logic to keep the value within correct vga boundaries
        VGACharacter {
            char_code: val,
            color_code: VGAColorCode::default(),
        }
    }
}

pub struct VGAWriter {
    base: &'static mut [VGACharacter],
    cols: u8,
    rows: u8,
    row_cursor: u8,
    col_cursor: u8,
}

impl VGAWriter {
    pub fn new() -> Self {
        let length = 80 * 25;
        let start = VGA_BUFFER_ADDR as *mut VGACharacter;
        let buf = core::ptr::slice_from_raw_parts_mut(start, length);

        VGAWriter {
            base: unsafe { &mut *(buf) },
            cols: 80,
            rows: 25,
            row_cursor: 0,
            col_cursor: 0,
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        let offset: usize = (self.row_cursor * self.cols + self.col_cursor).into();
        for (i, &b) in bytes.iter().enumerate() {
            self.col_cursor += 1;

            if self.col_cursor == self.cols {
                self.col_cursor = 0;
                self.row_cursor += 1;
            }

            if self.row_cursor == self.rows {
                // TODO: move lines up
                self.row_cursor = 0; // wrap around for now
            }

            self.base[offset + i] = b.into();
        }
    }
}
