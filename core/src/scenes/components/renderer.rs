use crate::{
    impl_component,
    scenes::{Any, Component},
};
use std::rc::Rc;
use three_d::Object;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Renderer {
    pub to_render: Rc<dyn Object>,
}

impl Renderer {
    pub fn new(to_render: impl Object + 'static) -> Self {
        Self { to_render: Rc::new(to_render) }
    }
}

impl_component!(Renderer);
