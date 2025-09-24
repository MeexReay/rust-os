#![no_std]
#![no_main]

mod vga;
mod keyboard;

use vga::{putc, puthex, puts, VGA_COLOR_BLACK, VGA_COLOR_MAGENTA, VGA_COLOR_WHITE};
use keyboard::{wait_scan, scan_to_symbol};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    puts(
        b"hello world from rust os",
        VGA_COLOR_BLACK,
        VGA_COLOR_MAGENTA,
        0,
        0
    );

    loop {
        let scan = wait_scan();
        let sym = scan_to_symbol(scan);
        puthex(scan, VGA_COLOR_BLACK, VGA_COLOR_WHITE, 13, 1);
        puthex(sym, VGA_COLOR_BLACK, VGA_COLOR_WHITE, 16, 1);
    }
}
