use ciri::{
    impl_component,
    scenes::{
        GameObject,
        components::{Component, Updateable},
    },
};
use std::{any::Any, fmt::Debug};

#[derive(Clone)]
pub struct Rotator {
    rotation_speed: f32,
}

impl Rotator {
    pub fn new(speed: f32) -> Self {
        Self { rotation_speed: speed }
    }
}

impl_component!(Rotator, updateable);

impl Updateable for Rotator {
    fn update(&mut self, delta_time: f32, game_object: &mut GameObject) {
        game_object.transform.rotate_y(self.rotation_speed * delta_time);
        println!("{:#?}", game_object.transform.rotation);
    }
}
