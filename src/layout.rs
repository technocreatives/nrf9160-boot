use crate::error::{FlashLayoutError, RamLayoutError};

const FLASH_SIZE: usize = 0x100_000; // 1 MiB
const RAM_END_ADDRESS: usize = 0x20_000_000 + 0x40_000; // 256 KiB

pub struct FlashLayout {
    pub(crate) bootloader: Section,
    pub(crate) application: Section,
}

pub struct RamLayout {
    pub(crate) bootloader: Section,
    pub(crate) modem_shared: Option<Section>,
    pub(crate) application: Section,
}

pub struct Section {
    pub address: usize,
    pub size: usize,
}

impl FlashLayout {
    pub const fn new(
        bootloader: Section,
        application: Section,
    ) -> Result<FlashLayout, FlashLayoutError> {
        if bootloader.address != 0x0 {
            return Err(FlashLayoutError::InvalidBootloaderStart);
        }

        if bootloader.address + bootloader.size > application.address {
            return Err(FlashLayoutError::SectionsOverlap);
        }

        if bootloader.size + application.size > FLASH_SIZE {
            return Err(FlashLayoutError::TooLarge);
        }

        Ok(FlashLayout {
            bootloader,
            application,
        })
    }
}

impl RamLayout {
    pub const fn new(
        bootloader: Section,
        application: Section,
        modem_shared: Option<Section>,
    ) -> Result<RamLayout, RamLayoutError> {
        match modem_shared {
            Some(modem_shared) => Self::shared(bootloader, application, modem_shared),
            None => Self::without_shared(bootloader, application),
        }
    }

    const fn without_shared(
        bootloader: Section,
        application: Section,
    ) -> Result<RamLayout, RamLayoutError> {
        let boot_end = bootloader.address + bootloader.size;
        let app_end = application.address + application.size;

        if bootloader.address < app_end && application.address < boot_end {
            return Err(RamLayoutError::SectionsOverlap);
        }

        if app_end > RAM_END_ADDRESS || boot_end > RAM_END_ADDRESS {
            return Err(RamLayoutError::TooLarge);
        }

        Ok(RamLayout {
            bootloader,
            modem_shared: None,
            application,
        })
    }

    const fn shared(
        bootloader: Section,
        application: Section,
        modem_shared: Section,
    ) -> Result<RamLayout, RamLayoutError> {
        let boot_end = bootloader.address + bootloader.size;
        let app_end = application.address + application.size;
        let modem_end = modem_shared.address + modem_shared.size;

        if bootloader.address < app_end && application.address < boot_end {
            return Err(RamLayoutError::SectionsOverlap);
        }

        if bootloader.address < modem_end && modem_shared.address < boot_end {
            return Err(RamLayoutError::SectionsOverlap);
        }

        if application.address < modem_end && modem_shared.address < app_end {
            return Err(RamLayoutError::SectionsOverlap);
        }

        if app_end > RAM_END_ADDRESS || boot_end > RAM_END_ADDRESS || modem_end > RAM_END_ADDRESS {
            return Err(RamLayoutError::TooLarge);
        }

        Ok(RamLayout {
            bootloader,
            modem_shared: Some(modem_shared),
            application,
        })
    }
}
