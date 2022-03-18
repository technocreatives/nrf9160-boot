use crate::error::LayoutError;
use bitflags::bitflags;

pub(crate) const FLASH_SIZE: usize = 0x10_0000; // 1 MiB
pub(crate) const RAM_START_ADDRESS: usize = 0x2000_0000;
pub(crate) const RAM_END_ADDRESS: usize = 0x2000_0000 + 0x4_0000; // 256 KiB

pub struct FlashLayout<'s> {
    pub(crate) sections: &'s [Section],
}

pub struct RamLayout<'s> {
    pub(crate) sections: &'s [Section],
}

#[derive(Clone, Copy)]
pub struct Section {
    pub address: usize,
    pub size: usize,
    pub permissions: SectionFlags,
}

bitflags! {
    pub struct SectionFlags: u32 {
        const EXECUTE = 0b00000001;
        const WRITE = 0b00000010;
        const READ = 0b00000100;
        const SECURE = 0b00010000;
    }
}

impl Default for SectionFlags {
    fn default() -> SectionFlags {
        SectionFlags::EXECUTE | SectionFlags::WRITE | SectionFlags::READ
    }
}

impl<'s> FlashLayout<'s> {
    pub fn new(sections: &'s [Section]) -> Result<FlashLayout, LayoutError> {
        for (n, section) in sections.iter().enumerate() {
            let section_end = section.address + section.size;
            if section_end > FLASH_SIZE {
                return Err(LayoutError::InvalidAddress);
            }
            for other_section in &sections[n + 1..] {
                let other_end = other_section.address + other_section.size;

                if section.address < other_end && other_section.address < section_end {
                    return Err(LayoutError::SectionsOverlap);
                }
            }
        }

        Ok(FlashLayout { sections })
    }
}

impl<'s> RamLayout<'s> {
    pub fn new(sections: &'s [Section]) -> Result<RamLayout, LayoutError> {
        for (n, section) in sections.iter().enumerate() {
            let section_end = section.address + section.size;

            if section.address < RAM_START_ADDRESS || section_end > RAM_END_ADDRESS {
                return Err(LayoutError::InvalidAddress);
            }

            for other_section in &sections[n + 1..] {
                let other_end = other_section.address + other_section.size;

                if section.address < other_end && other_section.address < section_end {
                    return Err(LayoutError::SectionsOverlap);
                }
            }
        }

        Ok(RamLayout { sections })
    }
}
