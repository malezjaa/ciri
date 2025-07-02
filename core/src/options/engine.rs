use three_d::{SurfaceSettings, WindowSettings};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineOptions {
    /// Window settings
    pub window: WindowSettings,
}

impl From<EngineOptions> for WindowSettings {
    fn from(options: EngineOptions) -> Self {
        options.window
    }
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineOptions {
    pub fn new() -> Self {
        Self {
            window: WindowSettings {
                title: "Engine".to_string(),
                min_size: (100, 100),
                max_size: None,
                initial_size: None,
                borderless: false,
                surface_settings: SurfaceSettings::default(),
            },
        }
    }
}
