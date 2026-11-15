
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryKind {
    Usable,
    Reserved,
    LoaderReclaimable,
}

#[derive(Clone, Copy, Debug)]
pub struct MemoryRegion {
    pub physical_start: u64,
    pub length: u64,
    pub kind: MemoryKind,
}

impl MemoryRegion {
    pub fn end_exclusive(&self) -> Option<u64> {
        self.physical_start.checked_add(self.length)
    }
}

pub struct BootInfo<'a> {
    pub memory_regions: &'a [MemoryRegion],
    pub platform_name: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_address_overflow() {
        let region = MemoryRegion {
            physical_start: u64::MAX,
            length: 2,
            kind: MemoryKind::Reserved,
        };
        assert_eq!(region.end_exclusive(), None);
    }
}
