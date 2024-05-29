use axvm::{AxvmHal, HostPhysAddr, HostVirtAddr};

const PAGE_SIZE: usize = 0x1000;

pub struct AxvmHalImpl;

impl AxvmHal for AxvmHalImpl {
    fn alloc_page() -> Option<HostPhysAddr> {
        axalloc::global_allocator()
            .alloc_pages(1, PAGE_SIZE)
            .map(|vaddr| axhal::mem::virt_to_phys(vaddr.into()))
            .ok()
    }

    fn dealloc_page(paddr: HostPhysAddr) {
        axalloc::global_allocator().dealloc_pages(axhal::mem::phys_to_virt(paddr).as_usize(), 1)
    }

    fn phys_to_virt(paddr: HostPhysAddr) -> HostVirtAddr {
        axhal::mem::phys_to_virt(paddr)
    }

    fn virt_to_phys(vaddr: HostVirtAddr) -> HostPhysAddr {
        axhal::mem::virt_to_phys(vaddr)
    }
}
