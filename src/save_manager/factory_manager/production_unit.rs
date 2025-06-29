use crate::base::manual_definition::{buildings::BuildingEnum, items::AveragedItemIO, recipes::RecipeEnum};
pub use crate::base::manual_definition::*;

/// Only one building type with a single recipe, but multiple times if needed, is connected to other production units
#[derive(Clone, Debug, PartialEq)]
pub struct ProductionUnit {
    name: String,
    building: BuildingEnum,
    recipe: RecipeEnum,
    input: Vec<AveragedItemIO>,
    output: Vec<AveragedItemIO>,
}



