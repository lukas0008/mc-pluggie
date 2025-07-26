use std::sync::Arc;

use crate::world_generator::WorldGenerator;

#[derive(Clone)]
pub struct WorldContext(pub Arc<WorldContextFunctions>);

pub struct WorldContextFunctions {
    pub add_generator: Box<dyn Fn(Box<dyn WorldGenerator>)>,
    pub get_generator: Box<dyn Fn(&str) -> Option<Box<dyn WorldGenerator>>>,
    pub list_generators: Box<dyn Fn() -> Vec<Box<dyn WorldGenerator>>>,
}

impl WorldContext {
    pub fn add_generator(&self, generator: impl WorldGenerator + 'static) {
        (self.0.add_generator)(Box::new(generator));
    }

    pub fn list_generators(&self) -> Vec<Box<dyn WorldGenerator>> {
        (self.0.list_generators)()
    }

    pub fn get_generator(&self, name: &str) -> Option<Box<dyn WorldGenerator>> {
        (self.0.get_generator)(name)
    }
}
