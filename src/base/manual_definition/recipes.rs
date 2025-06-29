use crate::base::manual_definition::items::RawItemIO;

use super::items::ItemEnum;
use super::buildings::BuildingEnum;


#[derive(Clone, PartialEq, Debug)]
pub struct Recipe {
    variant: RecipeEnum,
    name: String,
    input: Vec<RawItemIO>,
    output: Vec<RawItemIO>,
    // No manual machines
    machine: BuildingEnum,
    time: f32,
}

impl Recipe {
    pub fn new(name: &str, input: Vec<RawItemIO>, output: Vec<RawItemIO>, machine: BuildingEnum, time: f32, variant: RecipeEnum) -> Recipe {
        Recipe { variant, name: name.to_owned(), input, output, machine, time }
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum RecipeEnum {
    Iron2Ingot
}

impl RecipeEnum {
    pub fn get_recipe(&self) -> Recipe {
        match self {
            Self::Iron2Ingot => {
                let name = "Iron Smelting";
                let input = vec![RawItemIO::new(ItemEnum::IronOre, 1)];       // Not accurate
                let output = vec![RawItemIO::new(ItemEnum::IronIngot, 1)];    // Not accurate
                let machine = BuildingEnum::Smelter;
                let time = 10.;
                
                Recipe::new(name, input, output, machine, time, *self)
            },
        }
    }
}