use crate::base::manual_definition::{buildings::Building, items::AveragedItemIO};
use production_unit::*;

pub mod production_unit;

/// Group of factories in the same place but with no interconnected buildings
#[derive(Clone, Debug, PartialEq)]
pub struct MegaFactory {
    name: String,
    factories: Vec<Factory>,
}

impl MegaFactory {
    pub fn new(name: String, factories: Vec<Factory>) -> Self {
        MegaFactory { name, factories }
    }
}

/// Group of interconnected production units, can be seen as a production line
#[derive(Clone, Debug, PartialEq)]
pub struct Factory {
    name: String,
    production_units: Vec<ProductionUnit>,
    /// For instance to account for Awesome sinks power consumption
    additional_buildings: Vec<Building>,
    input: Vec<AveragedItemIO>,
    output: Vec<AveragedItemIO>,
}

impl Factory {
    pub fn new(name: String, production_units: Vec<ProductionUnit>, additional_buildings: Vec<Building>) -> Self {
        todo!()
    }
    
    /// Checks if connections between production units are coherent
    fn check_coherence(&self) -> bool {
        todo!()
    }
}



