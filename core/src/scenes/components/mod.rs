mod renderer;

use crate::scenes::GameObject;
use std::any::{Any, TypeId};

pub trait Component: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn clone_component(&self) -> Box<dyn Component>;
}

impl GameObject {
    pub fn add_component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, Box::new(component));
        self
    }

    pub fn get_component<T: Component + 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id)?.as_any().downcast_ref::<T>()
    }

    pub fn get_component_mut<T: Component + 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id)?.as_any_mut().downcast_mut::<T>()
    }

    pub fn remove_component<T: Component + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.remove(&type_id);
    }

    pub fn has_component<T: Component + 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.components.contains_key(&type_id)
    }

    pub fn get_component_types(&self) -> Vec<TypeId> {
        self.components.keys().copied().collect()
    }

    pub fn with_component<T: Component + 'static>(mut self, component: T) -> Self {
        self.add_component(component);
        self
    }
}

#[macro_export]
macro_rules! impl_component {
    ($type:ty) => {
        impl Component for $type {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn clone_component(&self) -> Box<dyn Component> {
                Box::new(self.clone())
            }
        }
    };
}

pub use renderer::*;
