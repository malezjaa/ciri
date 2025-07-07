mod ambient;
mod directional;

use crate::scenes::Scene;
pub use ambient::*;
pub use directional::*;
use std::sync::Arc;
use three_d::Light;

impl Scene {
    pub fn add_light(&mut self, light: impl Light + Send + Sync + 'static) {
        self.lights.push(Arc::new(light));
    }
}
