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
    summarize_regions(
        regions
            .iter()
            .map(|region| (region.kind, region.start, region.end)),
    )
}

fn summarize_regions(
    regions: impl IntoIterator<Item = (MemoryRegionKind, u64, u64)>,
) -> MemorySummary {
    let mut summary = MemorySummary {
        usable_bytes: 0,
        reserved_bytes: 0,
        region_count: 0,
    };

    for (kind, start, end) in regions {
        let size = end.saturating_sub(start);
        match kind {
            MemoryRegionKind::Usable => {
                summary.usable_bytes = summary.usable_bytes.saturating_add(size);
            }
            _ => {
                summary.reserved_bytes = summary.reserved_bytes.saturating_add(size);
            }
        }
        summary.region_count += 1;
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::{MemoryRegionKind, MemorySummary, summarize_regions};

    #[test]
    fn summarizes_usable_reserved_and_region_count() {
        let summary = summarize_regions([
            (MemoryRegionKind::Usable, 0, 1024),
            (MemoryRegionKind::Bootloader, 1024, 1536),
            (MemoryRegionKind::Usable, 4096, 8192),
        ]);

        assert_eq!(
            summary,
            MemorySummary {
                usable_bytes: 5120,
                reserved_bytes: 512,
                region_count: 3,
            }
        );
    }

    #[test]
    fn empty_memory_map_has_zero_totals() {
        assert_eq!(
            summarize_regions(core::iter::empty::<(MemoryRegionKind, u64, u64)>()),
            MemorySummary {
                usable_bytes: 0,
                reserved_bytes: 0,
                region_count: 0,
            }
        );
    }

    #[test]
    fn invalid_ranges_and_overflow_are_handled_safely() {
        let summary = summarize_regions([
            (MemoryRegionKind::Usable, 10, 5),
            (MemoryRegionKind::Usable, 0, u64::MAX),
            (MemoryRegionKind::Usable, 0, 1),
        ]);

        assert_eq!(summary.usable_bytes, u64::MAX);
        assert_eq!(summary.reserved_bytes, 0);
        assert_eq!(summary.region_count, 3);
    }
}
