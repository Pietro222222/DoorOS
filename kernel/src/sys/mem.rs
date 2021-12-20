use crate::api::console::CONSOLE as console;
pub const HEAP_START: usize = 0x4444_4444_0000;
use crate::System;
use bootloader::bootinfo::MemoryRegionType;
use bootloader::{bootinfo::MemoryMap, BootInfo};
//use core::cmp::min;
use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::instructions::interrupts;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
    Translate,
};
use x86_64::{PhysAddr, VirtAddr};
pub static TOTAL_MEM: AtomicU64 = AtomicU64::new(0);
pub static mut PHYS_MEM_OFFSET: u64 = 0;
pub static mut MEMORY_MAP: Option<&MemoryMap> = None;
pub fn init(info: &'static BootInfo) {
    interrupts::without_interrupts(|| {
        console.log("[INFO] Started Allocation...");
        let mut memory_size = 0;
        for region in info.memory_map.iter() {
            let start_addr = region.range.start_addr();
            let end_addr = region.range.end_addr();
            memory_size += end_addr - start_addr;
        }
        TOTAL_MEM.store(memory_size, Ordering::Relaxed);
        console.log("[INFO] Allocator is Allocating memory...");
        unsafe { PHYS_MEM_OFFSET = info.physical_memory_offset };
        unsafe { MEMORY_MAP.replace(&info.memory_map) };
        let mut mapper = unsafe { mapper(VirtAddr::new(PHYS_MEM_OFFSET)) };
        let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&info.memory_map) };
        init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
        console.log(&alloc::format!(
            "[INFO] Total of Memory: {}KB or {}MB",
            memory_size >> 10,
            memory_size >> 20
        ));
        console.log("[INFO] Allocation Completed!");
    });
}
pub fn phys_to_virt(addr: PhysAddr) -> VirtAddr {
    VirtAddr::new(addr.as_u64() + unsafe { PHYS_MEM_OFFSET })
}

pub fn virt_to_phys(addr: VirtAddr) -> Option<PhysAddr> {
    let mapper = unsafe { mapper(VirtAddr::new(PHYS_MEM_OFFSET)) };
    mapper.translate_addr(addr)
}

pub unsafe fn mapper(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let heap_size: u64 = TOTAL_MEM.load(Ordering::Relaxed);
    let pages = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + heap_size - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    let frame = frame_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
    unsafe {
        mapper.map_to(pages.start, frame, flags, frame_allocator)?.flush();
    }

    unsafe {
        System.lock().init(HEAP_START, heap_size as usize);
    }
    Ok(())
}
