#![no_main]
#![no_std]

use core::panic::PanicInfo;
use nrf9160_boot::{
    config_flash, config_peripherals, config_ram, jump, layout::SectionFlags, FlashLayout,
    RamLayout, Section,
};

const BOOT_FLASH: Section = Section {
    address: 0x0,
    size: 0x4_0000,
    permissions: SectionFlags::from_bits_truncate(
        SectionFlags::EXECUTE.bits() | SectionFlags::READ.bits() | SectionFlags::SECURE.bits(),
    ),
};

const APP_FLASH: Section = Section {
    address: 0x4_0000,
    size: 0xC_0000,
    permissions: SectionFlags::from_bits_truncate(
        SectionFlags::EXECUTE.bits() | SectionFlags::READ.bits() | SectionFlags::WRITE.bits(),
    ),
};

const BOOT_RAM: Section = Section {
    address: 0x2000_0000,
    size: 0x1_0000,
    permissions: SectionFlags::from_bits_truncate(
        SectionFlags::EXECUTE.bits()
            | SectionFlags::READ.bits()
            | SectionFlags::WRITE.bits()
            | SectionFlags::SECURE.bits(),
    ),
};

const MODEM_RAM: Section = Section {
    address: 0x2001_0000,
    size: 0x1_0000,
    permissions: SectionFlags::from_bits_truncate(
        SectionFlags::EXECUTE.bits() | SectionFlags::READ.bits() | SectionFlags::WRITE.bits(),
    ),
};

const APP_RAM: Section = Section {
    address: 0x2002_0000,
    size: 0x2_0000,
    permissions: SectionFlags::from_bits_truncate(
        SectionFlags::EXECUTE.bits() | SectionFlags::READ.bits() | SectionFlags::WRITE.bits(),
    ),
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = nrf9160_pac::Peripherals::take().unwrap();
    let flash_layout = FlashLayout::new(&[BOOT_FLASH, APP_FLASH]).unwrap();
    let ram_layout = RamLayout::new(&[BOOT_RAM, MODEM_RAM, APP_RAM]).unwrap();

    config_flash(&p.SPU_S, &flash_layout);
    config_ram(&p.SPU_S, &ram_layout);
    config_peripherals(p);

    unsafe { jump(APP_FLASH.address as u32) }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        cortex_m::asm::wfe();
    }
}
