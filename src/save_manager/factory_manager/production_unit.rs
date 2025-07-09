use crate::base::manual_definition::{buildings::Building, items::MultipleItemsIO, recipes::Recipe};

/// Only one building type with a single recipe, but multiple times if needed, is connected to other production units
#[derive(Clone, Debug, PartialEq)]
pub struct ProductionUnit {
    name: String,
    building: Building,
    recipe: Recipe,
    input: MultipleItemsIO,
    output: MultipleItemsIO,
}

impl ProductionUnit {
    pub fn new(name: String, building: Building, recipe: Recipe) -> Self {
        todo!()
    }
}




