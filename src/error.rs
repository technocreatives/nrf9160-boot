#[derive(Debug, Clone, Copy)]
pub enum FlashLayoutError {
    InvalidBootloaderStart,
    SectionsOverlap,
    TooLarge,
}

#[derive(Debug, Clone, Copy)]
pub enum RamLayoutError {
    SectionsOverlap,
    TooLarge,
}
