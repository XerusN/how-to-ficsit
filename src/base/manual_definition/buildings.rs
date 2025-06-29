use super::super::power::Power;

/// Does not define miners or other sources building
#[derive(Clone, PartialEq, Debug)]
pub struct Building {
    variant: BuildingEnum,
    name: String,
    consumption: Power,
}

impl Building {
    pub fn new(name: &str, consumption: Power, variant: BuildingEnum) -> Self {
        Building { variant, name: name.to_owned(), consumption }
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum BuildingEnum {
    Smelter
}

impl BuildingEnum {
    pub fn get_buidling(&self) -> Building {
        match self {
            Self::Smelter => {
                let name = "Smelter";
                let consumption = Power(50.);       // Not accurate
                
                Building::new(name, consumption, *self)
            },
        }
    }
}