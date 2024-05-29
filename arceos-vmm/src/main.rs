#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(naked_functions)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

#[macro_use]
extern crate log;

mod hal;
mod vmexit;

use axvm::AxvmPerCpu;

use self::hal::AxvmHalImpl;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Starting virtualization...");
    println!("Hardware support: {:?}", axvm::has_hardware_support());

    let mut percpu = AxvmPerCpu::<AxvmHalImpl>::new(0);
    percpu
        .hardware_enable()
        .expect("Failed to enable virtualization");
    let mut vcpu = percpu.create_vcpu(test_guest as usize).unwrap();
    println!("{:#x?}", vcpu);
    println!("Running guest...");
    vcpu.run();
}

#[naked]
unsafe extern "C" fn test_guest() -> ! {
    core::arch::asm!(
        "
        mov     rax, 0
        mov     rdi, 2
        mov     rsi, 3
        mov     rdx, 3
        mov     rcx, 3
    2:
        vmcall
        add     rax, 1
        jmp     2b",
        options(noreturn),
    );
}
