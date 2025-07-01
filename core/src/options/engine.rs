use three_d::{SurfaceSettings, Window, WindowSettings};

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

impl EngineOptions {
    pub fn builder() -> EngineOptionsBuilder {
        EngineOptionsBuilder::new()
    }
}

pub struct EngineOptionsBuilder {
    window: WindowSettings,
}

impl EngineOptionsBuilder {
    pub fn with_name(mut self, name: &str) -> Self {
        self.window.title = name.to_string();
        self
    }

    pub fn min_size(mut self, width: u32, height: u32) -> Self {
        self.window.min_size = (width, height);
        self
    }

    pub fn max_size(mut self, width: u32, height: u32) -> Self {
        self.window.max_size = Some((width, height));
        self
    }

    pub fn initial_size(mut self, width: u32, height: u32) -> Self {
        self.window.initial_size = Some((width, height));
        self
    }

    pub fn borderless(mut self, borderless: bool) -> Self {
        self.window.borderless = borderless;
        self
    }

    pub fn surface_settings(mut self, settings: SurfaceSettings) -> Self {
        self.window.surface_settings = settings;
        self
    }

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

    pub fn build(self) -> EngineOptions {
        EngineOptions { window: self.window }
    }
}
