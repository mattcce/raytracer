pub trait Preset: Default {
    fn preset() -> Self {
        Self::default()
    }
}
