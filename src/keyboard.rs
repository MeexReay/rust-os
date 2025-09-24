use core::arch::asm;

const STATUS_REG: usize = 0x64;
const DATA_PORT: usize = 0x60;

pub unsafe fn inportb(port: usize) -> u8 {
    let mut value: u8;
    asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
    );
    value
}

pub unsafe fn has_data() -> bool {
    inportb(STATUS_REG) & 1 != 0
}

pub unsafe fn read_scan() -> u8 {
    inportb(DATA_PORT)
}

pub unsafe fn wait_scan() -> u8 {
    while !has_data() {};
    read_scan()
}

const SCAN_TO_SYM_TABLE: &[u8] = b"\0\x271234567890-=\x80\tqwertyuiop[]\n\0asdfghjkl;\'`\0\\zxcvbnm,./\0*\0 ";

pub fn scan_to_symbol(code: u8) -> u8 {
    if code < SCAN_TO_SYM_TABLE.len() as u8 {
        SCAN_TO_SYM_TABLE[code as usize]
    } else {
        0
    }
}

