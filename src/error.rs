#[derive(Debug, Clone, Copy)]
pub enum LayoutError {
    SectionsOverlap,
    InvalidAddress,
}
