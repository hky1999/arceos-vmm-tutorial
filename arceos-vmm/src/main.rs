#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(naked_functions)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

extern crate alloc;

#[macro_use]
extern crate log;

mod gconfig;
mod gpm;
mod hal;
mod vmexit;

use axerrno::AxResult;
use axhal::mem::virt_to_phys;
use axvm::{AxvmPerCpu, GuestPhysAddr, HostVirtAddr};
use page_table_entry::MappingFlags;

use self::gconfig::*;
use self::gpm::{GuestMemoryRegion, GuestPhysMemorySet};
use self::hal::AxvmHalImpl;

#[repr(align(4096))]
struct AlignedMemory<const LEN: usize>([u8; LEN]);

static mut GUEST_PHYS_MEMORY: AlignedMemory<GUEST_PHYS_MEMORY_SIZE> =
    AlignedMemory([0; GUEST_PHYS_MEMORY_SIZE]);

fn gpa_as_mut_ptr(guest_paddr: GuestPhysAddr) -> *mut u8 {
    let offset = unsafe { core::ptr::addr_of!(GUEST_PHYS_MEMORY) as *const _ as usize };
    let host_vaddr = guest_paddr + offset;
    host_vaddr as *mut u8
}

fn setup_guest_page_table() {
    use x86_64::structures::paging::{PageTable, PageTableFlags as PTF};
    let pt1 = unsafe { &mut *(gpa_as_mut_ptr(GUEST_PT1) as *mut PageTable) };
    let pt2 = unsafe { &mut *(gpa_as_mut_ptr(GUEST_PT2) as *mut PageTable) };
    // identity mapping
    pt1[0].set_addr(
        x86_64::PhysAddr::new(GUEST_PT2 as _),
        PTF::PRESENT | PTF::WRITABLE,
    );
    pt2[0].set_addr(
        x86_64::PhysAddr::new(0),
        PTF::PRESENT | PTF::WRITABLE | PTF::HUGE_PAGE,
    );
}

fn setup_gpm() -> AxResult<GuestPhysMemorySet> {
    setup_guest_page_table();

    // copy guest code
    unsafe {
        core::ptr::copy_nonoverlapping(
            test_guest as usize as *const u8,
            gpa_as_mut_ptr(GUEST_ENTRY),
            0x100,
        );
    }

    // create nested page table and add mapping
    let mut gpm = GuestPhysMemorySet::new()?;
    let guest_memory_regions = [GuestMemoryRegion {
        // RAM
        gpa: GUEST_PHYS_MEMORY_BASE,
        hpa: virt_to_phys(HostVirtAddr::from(
            gpa_as_mut_ptr(GUEST_PHYS_MEMORY_BASE) as usize
        )),
        size: GUEST_PHYS_MEMORY_SIZE,
        flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
    }];
    for r in guest_memory_regions.into_iter() {
        trace!("{:#x?}", r);
        gpm.map_region(r.into())?;
    }
    Ok(gpm)
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Starting virtualization...");
    info!("Hardware support: {:?}", axvm::has_hardware_support());

    let mut percpu = AxvmPerCpu::<AxvmHalImpl>::new(0);
    percpu
        .hardware_enable()
        .expect("Failed to enable virtualization");

    let gpm = setup_gpm().expect("Failed to set guest physical memory set");
    info!("{:#x?}", gpm);
    let mut vcpu = percpu
        .create_vcpu(GUEST_ENTRY, gpm.nest_page_table_root())
        .expect("Failed to create vcpu");
    vcpu.set_page_table_root(GUEST_PT1);
    vcpu.set_stack_pointer(GUEST_STACK_TOP);
    info!("{:#x?}", vcpu);

    println!("Running guest...");

    vcpu.run();
}

unsafe extern "C" fn test_guest() -> ! {
    for i in 0..100 {
        core::arch::asm!(
            "vmcall",
            inout("rax") i => _,
            in("rdi") 2,
            in("rsi") 3,
            in("rdx") 3,
            in("rcx") 3,
        );
    }
    core::arch::asm!("mov qword ptr [$0xffff233], $2333"); // panic
    loop {}
}
