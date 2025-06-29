use core::option::Option::None;

use super::super::power::Power;

/// Does not define miners or other sources building
#[derive(Clone, PartialEq)]
pub struct Source {
    variant: SourceEnum,
    name: String,
    consumption: Power,
}

impl Source {
    pub fn new(name: &str, consumption: Power, variant: SourceEnum) -> Self {
        Source { variant, name: name.to_owned(), consumption }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum SourceEnum {
    Mine(f32),
}

impl SourceEnum {
    pub fn get_buidling(&self) -> Source {
        match self {
            Self::Mine(_) => {
                let name = "Mine";
                let consumption = Power(50.);       // Not accurate
                
                Source::new(name, consumption, *self)
            },
        }
    }
}