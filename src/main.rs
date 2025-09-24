#![no_std]
#![no_main]

mod vga;

use vga::{puts, VGA_COLOR_BLACK, VGA_COLOR_MAGENTA};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        puts(
            "hello world from rust os",
            VGA_COLOR_BLACK,
            VGA_COLOR_MAGENTA,
            0,
            0
        );
    }
    loop {}
}
