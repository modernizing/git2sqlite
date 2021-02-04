#[derive(Debug, Clone)]
pub struct ConvertOptions {
    pub with_changes: bool,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        ConvertOptions {
            with_changes: false
        }
    }
}
