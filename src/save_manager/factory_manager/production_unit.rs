use crate::base::manual_definition::{buildings::BuildingEnum, items::MultipleItemsIO, recipes::RecipeEnum};

/// Only one building type with a single recipe, but multiple times if needed, is connected to other production units
#[derive(Clone, Debug, PartialEq)]
pub struct ProductionUnit {
    name: String,
    building: BuildingEnum,
    recipe: RecipeEnum,
    input: MultipleItemsIO,
    output: MultipleItemsIO,
}

impl ProductionUnit {
    pub fn new(name: String, building: BuildingEnum, recipe: RecipeEnum) -> Self {
        todo!()
    }
}




