#![no_std]
#![no_main]

use syscalls::{Sysno, raw_syscall};

#[no_mangle]
unsafe extern "C" fn _start() {
    raw_syscall!(Sysno::exit, 0);
}

#[panic_handler]
unsafe fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    core::hint::unreachable_unchecked();
}
