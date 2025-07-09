mod renderer;
mod traits;
pub use traits::*;

use crate::scenes::game_object::GameObject;
pub use crate::{
    frame::Frame,
    scenes::{
        Scene,
        components::traits::{ComponentRequirements, Updateable},
    },
};
pub use renderer::*;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    fmt::Debug,
    ptr::NonNull,
};

pub trait Component: Any + Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn clone_component(&self) -> Box<dyn Component>;

    fn as_updateable(&mut self) -> Option<&mut dyn Updateable> {
        None
    }

    fn get_requirements(&self) -> ComponentRequirements {
        ComponentRequirements::default()
    }
}

impl GameObject {
    pub fn add_component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        let requirements = component.get_requirements();

        for required in requirements.required {
            if !self.has_component_by_id(required) {
                panic!("missing required component");
            }
        }

        for conflict in requirements.conflicts {
            if self.has_component_by_id(conflict) {
                panic!("conflicting component exists");
            }
        }

        self.components.insert(TypeId::of::<T>(), component.clone_component());
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

    pub fn has_component_by_id(&self, type_id: TypeId) -> bool {
        self.components.contains_key(&type_id)
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

        impl Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", stringify!($type))
            }
        }
    };

    ($type:ty, updateable) => {
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

            fn as_updateable(&mut self) -> Option<&mut dyn Updateable> {
                Some(self)
            }
        }

        impl Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", stringify!($type))
            }
        }
    };
}

impl Scene {
    pub fn update(&mut self, delta_time: f32) {
        for (_, object) in self.objects.iter_mut() {
            if !object.active {
                continue;
            }

            let object_ptr = object as *mut GameObject;

            let updateables: Vec<_> = object
                .components
                .values_mut()
                .filter_map(|wrapper| wrapper.as_updateable())
                .collect();

            for updateable in updateables {
                unsafe {
                    updateable.update(delta_time, &mut *object_ptr);
                }
            }
        }
    }
}
