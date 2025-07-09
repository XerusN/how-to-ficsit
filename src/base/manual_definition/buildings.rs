use super::super::power::Power;

// Macro to generate getter functions for Item
macro_rules! generate_building_getters {
    (
        $(
            $variant:ident => {
                name: $name:expr,
                consumption: $consumption:expr,
            }
        ),* $(,)?
    ) => {
        impl Building {
            pub fn get_name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $name,
                    )*
                }
            }

            pub fn get_consumption(&self) -> Power {
                match self {
                    $(
                        Self::$variant => $consumption,
                    )*
                }
            }
        }
    };
}

// Add data about an item here
generate_building_getters! {
    Smelter => {
        name: "Smelter",
        consumption: Power(50.),    // Not accurate
    },
    Miner => {
        name: "Miner",
        consumption: Power(50.),    // Not accurate
    },
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Building {
    Smelter,
    Miner,
}