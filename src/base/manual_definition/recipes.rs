use crate::base::manual_definition::items::RawItemIO;

use super::items::Item;
use super::buildings::Building;

// Macro to generate getter functions for Item
macro_rules! generate_recipe_getters {
    (
        $(
            $variant:ident => {
                name: $name:expr,
                input: $input:expr,
                output: $output:expr,
                machine: $machine:expr,
                time: $time:expr,
            }
        ),* $(,)?
    ) => {
        impl Recipe {
            pub fn get_name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $name,
                    )*
                }
            }
            
            pub fn get_input(&self) -> Vec<RawItemIO> {
                match self {
                    $(
                        Self::$variant => $input,
                    )*
                }
            }
            
            pub fn get_output(&self) -> Vec<RawItemIO> {
                match self {
                    $(
                        Self::$variant => $output,
                    )*
                }
            }

            pub fn get_machine(&self) -> Building {
                match self {
                    $(
                        Self::$variant => $machine,
                    )*
                }
            }
            
            pub fn get_time(&self) -> f64 {
                match self {
                    $(
                        Self::$variant => $time,
                    )*
                }
            }
        }
    };
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Recipe {
    IronMining,
    IronSmelting,
}

// Add data about an item here
generate_recipe_getters! {
    IronMining => {
        name: "Iron Mining",
        input: vec![],
        output: vec![RawItemIO::new(Item::IronOre, 1)],     // Not accurate
        machine: Building::Miner,
        time: 10.,                                          // Not accurate
    },
    IronSmelting => {
        name: "Iron Smelting",
        input: vec![RawItemIO::new(Item::IronOre, 1)],      // Not accurate
        output: vec![RawItemIO::new(Item::IronIngot, 1)],   // Not accurate
        machine: Building::Smelter,
        time: 10.,                                          // Not accurate
    },
}
