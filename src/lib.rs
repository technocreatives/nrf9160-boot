#![no_std]

use nrf9160_pac::{Peripherals, SPU_S};

pub use layout::{FlashLayout, RamLayout, Section};

pub mod error;
pub mod layout;

const FLASH_REGION_SIZE: usize = 0x8_000; // 32 KiB;
const RAM_REGION_SIZE: usize = 0x2_000; // 8 KiB;
const RAM_START_ADDRESS: usize = 0x20_000_000;

pub fn config_flash(spu: &SPU_S, layout: &FlashLayout) {
    let bootloader_sections = layout.bootloader.size / FLASH_REGION_SIZE;
    for n in 0..bootloader_sections {
        spu.flashregion[n].perm.write(|w| {
            w.read()
                .enable()
                .write()
                .enable()
                .secattr()
                .secure()
                .execute()
                .enable()
                .lock()
                .locked()
        });
    }

    let app_section_start = layout.application.address / FLASH_REGION_SIZE;
    let app_section_end = (layout.application.size / FLASH_REGION_SIZE) + app_section_start;
    for n in app_section_start..app_section_end {
        spu.flashregion[n].perm.write(|w| {
            w.read()
                .enable()
                .write()
                .enable()
                .secattr()
                .non_secure()
                .execute()
                .enable()
                .lock()
                .locked()
        });
    }
}

pub fn config_ram(spu: &SPU_S, layout: &RamLayout) {
    let boot_section_start = (layout.bootloader.address - RAM_START_ADDRESS) / RAM_REGION_SIZE;
    let boot_section_end = (layout.bootloader.size / RAM_REGION_SIZE) + boot_section_start;
    for n in boot_section_start..boot_section_end {
        spu.ramregion[n].perm.write(|w| {
            w.read()
                .enable()
                .write()
                .enable()
                .secattr()
                .secure()
                .execute()
                .enable()
                .lock()
                .locked()
        });
    }

    let app_section_start = (layout.application.address - RAM_START_ADDRESS) / RAM_REGION_SIZE;
    let app_section_end = (layout.application.size / RAM_REGION_SIZE) + app_section_start;
    for n in app_section_start..app_section_end {
        spu.flashregion[n].perm.write(|w| {
            w.read()
                .enable()
                .write()
                .enable()
                .secattr()
                .non_secure()
                .execute()
                .enable()
                .lock()
                .locked()
        });
    }

    if let Some(ref modem_shared) = layout.modem_shared {
        let modem_section_start = (modem_shared.address - RAM_START_ADDRESS) / RAM_REGION_SIZE;
        let modem_section_end = (modem_shared.size / RAM_REGION_SIZE) + modem_section_start;

        for n in modem_section_start..modem_section_end {
            spu.flashregion[n].perm.write(|w| {
                w.read()
                    .enable()
                    .write()
                    .enable()
                    .secattr()
                    .non_secure()
                    .execute()
                    .enable()
                    .lock()
                    .locked()
            });
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

union Vector {
    pub handler: unsafe extern "C" fn(),
    pub reserved: usize,
}

pub unsafe fn jump(layout: &FlashLayout) -> ! {
    let scb = &*nrf9160_pac::SCB::ptr();
    scb.vtor.write(layout.application.address as u32);

    let vector_table = layout.application.address as *const Vector;
    let msp = (*vector_table.offset(0)).handler as *const u32;
    let rsv = vector_table.offset(1);

    let ns_rsv = Vector {
        reserved: (((*rsv).handler as usize) & !1) as usize,
    };
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    cortex_m::asm::bootstrap(msp, ns_rsv.handler as *const u32);
}
