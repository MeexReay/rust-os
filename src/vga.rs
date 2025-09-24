pub const VGA_COLOR_BLACK: u8 = 0;
pub const VGA_COLOR_BLUE: u8 = 1;
pub const VGA_COLOR_GREEN: u8 = 2;
pub const VGA_COLOR_CYAN: u8 = 3;
pub const VGA_COLOR_RED: u8 = 4;
pub const VGA_COLOR_MAGENTA: u8 = 5;
pub const VGA_COLOR_BROWN: u8 = 6;
pub const VGA_COLOR_LIGHT_GREY: u8 = 7;
pub const VGA_COLOR_DARK_GREY: u8 = 8;
pub const VGA_COLOR_LIGHT_BLUE: u8 = 9;
pub const VGA_COLOR_LIGHT_GREEN: u8 = 10;
pub const VGA_COLOR_LIGHT_CYAN: u8 = 11;
pub const VGA_COLOR_LIGHT_RED: u8 = 12;
pub const VGA_COLOR_LIGHT_MAGENTA: u8 = 13;
pub const VGA_COLOR_LIGHT_BROWN: u8 = 14;
pub const VGA_COLOR_WHITE: u8 = 15;

pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;
pub const VGA_MEMORY: *mut u16 = 0xB8000 as *mut u16;

pub unsafe fn putc(ch: u8, bg: u8, fg: u8, x: usize, y: usize) {
    VGA_MEMORY.add(x + y * VGA_WIDTH).write(ch as u16 | ((fg | bg << 4) as u16) << 8);
}

pub unsafe fn puts(st: &[u8], bg: u8, fg: u8, x: usize, y: usize) {
    let mut nx = x;
    let mut ny = y;
    
    for c in st {
        putc(*c, bg, fg, nx, ny);
        nx += 1;
        if nx == VGA_WIDTH {
            nx = 0;
            ny += 1;
        }
    }
}

const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

pub unsafe fn puthex(byte: u8, bg: u8, fg: u8, x: usize, y: usize) {
    let to_char = |x| {
        match x {
            0..=9 => b'0' + x,
            10..=16 => b'A' + x - 10,
            _ => b'?'
        }
    };
    let high = (byte >> 4) & 0x0F;
    let low = byte & 0x0F;

    puts(&[0, to_char(high), 0, to_char(low)], bg, fg, x, y);
}
