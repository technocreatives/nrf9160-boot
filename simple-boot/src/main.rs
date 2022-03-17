#![no_main]
#![no_std]

extern crate panic_reset;

const BOOT_FLASH: Section = Section {
    address: 0x0,
    size: 0x40_000,
};
const APP_FLASH: Section = Section {
    address: 0x40_000,
    size: 0xC0_000,
};

const FLASH_LAYOUT: FlashLayout = match FlashLayout::new(BOOT_FLASH, APP_FLASH) {
    Ok(layout) => layout,
    Err(_) => panic!("Could not config flash"),
};

const BOOT_RAM: Section = Section {
    address: 0x20_000_000,
    size: 0x10_000,
};

const MODEM_RAM: Section = Section {
    address: 0x20_010_000,
    size: 0x10_000,
};

const APP_RAM: Section = Section {
    address: 0x20_020_000,
    size: 0x20_000,
};

const RAM_LAYOUT: RamLayout = match RamLayout::new(BOOT_RAM, APP_RAM, Some(MODEM_RAM)) {
    Ok(layout) => layout,
    Err(_) => panic!("Could not config ram"),
};

use nrf9160_boot::{
    config_flash, config_peripherals, config_ram, jump, FlashLayout, RamLayout, Section,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = nrf9160_pac::Peripherals::take().unwrap();

    config_flash(&p.SPU_S, &FLASH_LAYOUT);
    config_ram(&p.SPU_S, &RAM_LAYOUT);
    config_peripherals(p);
    
    unsafe {
        jump(&FLASH_LAYOUT)
    }
}
