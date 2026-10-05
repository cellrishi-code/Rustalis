//! Kernel memory foundations.
//!
//! This module currently owns boot-time memory-map inspection. Physical frame
//! allocation and page-table management will build on these types later.

use bootloader_api::info::{MemoryRegionKind, MemoryRegions};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemorySummary {
    pub usable_bytes: u64,
    pub reserved_bytes: u64,
    pub region_count: usize,
}

pub fn summarize(regions: &MemoryRegions) -> MemorySummary {
    let mut usable_bytes = 0;
    let mut reserved_bytes = 0;

    for region in regions.iter() {
        let size = region.end.saturating_sub(region.start);
        match region.kind {
            MemoryRegionKind::Usable => usable_bytes = usable_bytes.saturating_add(size),
            _ => reserved_bytes = reserved_bytes.saturating_add(size),
        }
    }

    MemorySummary {
        usable_bytes,
        reserved_bytes,
        region_count: regions.iter().count(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn saturating_arithmetic_is_safe_for_memory_counters() {
        assert_eq!(u64::MAX.saturating_add(1), u64::MAX);
    }
}
