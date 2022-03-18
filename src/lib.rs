#![no_std]

use layout::RAM_START_ADDRESS;
use nrf9160_pac::{Peripherals, SPU_S};

pub use layout::{FlashLayout, RamLayout, Section};

pub mod error;
pub mod layout;

const FLASH_REGION_SIZE: usize = 0x8000; // 32 KiB;
const RAM_REGION_SIZE: usize = 0x2000; // 8 KiB;

pub fn config_flash(spu: &SPU_S, layout: &FlashLayout) {
    for section in layout.sections {
        let start_region = section.address / FLASH_REGION_SIZE;
        let end_region = (section.size / FLASH_REGION_SIZE) + start_region;
        for n in start_region..end_region {
            spu.flashregion[n]
                .perm
                .write(|w| unsafe { w.bits(section.permissions.bits()).lock().locked() })
        }
    }
}

pub fn config_ram(spu: &SPU_S, layout: &RamLayout) {
    for section in layout.sections {
        let start_region = (section.address - RAM_START_ADDRESS) / RAM_REGION_SIZE;
        let end_region = (section.size / RAM_REGION_SIZE) + start_region;
        for n in start_region..end_region {
            spu.flashregion[n]
                .perm
                .write(|w| unsafe { w.bits(section.permissions.bits()).lock().locked() })
        }
    }
}

pub fn config_peripherals(p: Peripherals) {
    for n in 0..67 {
        if !p.SPU_S.periphid[n].perm.read().securemapping().is_secure() {
            p.SPU_S.periphid[n].perm.write(|w| w.secattr().non_secure());
        }
    }

    p.SPU_S.gpioport[0].perm.write(|w| {
        w.pin0()
            .non_secure()
            .pin1()
            .non_secure()
            .pin2()
            .non_secure()
            .pin3()
            .non_secure()
            .pin4()
            .non_secure()
            .pin5()
            .non_secure()
            .pin6()
            .non_secure()
            .pin7()
            .non_secure()
            .pin8()
            .non_secure()
            .pin9()
            .non_secure()
            .pin10()
            .non_secure()
            .pin11()
            .non_secure()
            .pin12()
            .non_secure()
            .pin13()
            .non_secure()
            .pin14()
            .non_secure()
            .pin15()
            .non_secure()
            .pin16()
            .non_secure()
            .pin17()
            .non_secure()
            .pin18()
            .non_secure()
            .pin19()
            .non_secure()
            .pin20()
            .non_secure()
            .pin21()
            .non_secure()
            .pin22()
            .non_secure()
            .pin23()
            .non_secure()
            .pin24()
            .non_secure()
            .pin25()
            .non_secure()
            .pin26()
            .non_secure()
            .pin27()
            .non_secure()
            .pin28()
            .non_secure()
            .pin29()
            .non_secure()
            .pin30()
            .non_secure()
            .pin31()
            .non_secure()
    });
}

/// Jump into non-secure application code.
///
/// # Safety
///
/// This function requires you pass an address pointing to the start of the application vector
/// table. Application ram must also have been configured as read, write, execute, and non-secure earlier
/// in the bootloader
pub unsafe fn jump(app_base: u32) -> ! {
    let scb = &*nrf9160_pac::SCB::ptr();
    scb.vtor.write(app_base);

    let vector_table = app_base as *const u32;
    let msp = *vector_table.offset(0) as *const u32;
    let rsv = vector_table.offset(1);

    let ns_rsv = (*rsv) & !1;
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    cortex_m::asm::bootstrap(msp, ns_rsv as *const u32);
}
