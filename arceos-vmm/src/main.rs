#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

mod hal;

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

    let mut vcpu = percpu.create_vcpu().unwrap();
    println!("{:#x?}", vcpu);
    vcpu.run();
}
