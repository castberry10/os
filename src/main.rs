#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)] 

use core::panic::PanicInfo;

mod vga_buffer;
mod interrupts; 

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    interrupts::init_idt(); 
    println!("IDT loaded.");

    x86_64::instructions::interrupts::int3();

    println!("It did not crash!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
