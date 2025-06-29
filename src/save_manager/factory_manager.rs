use crate::base::manual_definition::buildings::Building;
use production_unit::*;

pub mod production_unit;

#[derive(Clone, Debug, PartialEq)]
pub struct Factory {
    production_units: Vec<ProductionUnit>,
    additional_buildings: Vec<Building>,
    import: Vec
}