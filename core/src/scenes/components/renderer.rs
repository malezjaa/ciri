use crate::{
    impl_component,
    scenes::{Any, Component},
};
use three_d::Object;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct Renderer {
    pub to_render: Arc<dyn Object + Send + Sync> ,
}

impl Renderer {
    pub fn new(to_render: impl Object + Send + Sync + 'static) -> Self {
        Self { to_render: Arc::new(to_render) }
    }
}

impl_component!(Renderer);
