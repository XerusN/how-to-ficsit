use super::recipes::Recipe;

// Macro to generate getter functions for Item
macro_rules! generate_item_getters {
    (
        $(
            $variant:ident => {
                name: $name:expr,
                recipes: $recipes:expr,
                fluid: $fluid:expr
            }
        ),* $(,)?
    ) => {
        impl Item {
            pub fn get_name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $name,
                    )*
                }
            }

            pub fn get_recipes(&self) -> Vec<Recipe> {
                match self {
                    $(
                        Self::$variant => $recipes,
                    )*
                }
            }

            pub fn get_fluid(&self) -> bool {
                match self {
                    $(
                        Self::$variant => $fluid,
                    )*
                }
            }
        }
    };
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Item {
    IronOre,
    IronIngot,
}

// Add data about an item here
generate_item_getters! {
    IronOre => {
        name: "Iron Ore",
        recipes: vec![],
        fluid: false
    },
    IronIngot => {
        name: "Iron Ingot",
        recipes: vec![Recipe::IronSmelting],
        fluid: false
    },
}

/// Items used in a recipe for example, always an integer
#[derive(Clone, PartialEq, Debug, Copy)]
pub struct RawItemIO {
    item: Item,
    io: u32,
}

impl RawItemIO {
    pub fn new(item: Item, io: u32) -> Self {
        RawItemIO { item, io }
    }
}

/// Items produced or used per minute
#[derive(Clone, PartialEq, Debug, Copy)]
pub struct AveragedItemIO {
    item: Item,
    io: f32,
}

impl AveragedItemIO {
    pub fn new(item: Item, io: f32) -> Self {
        AveragedItemIO { item, io }
    }
    
    pub fn item(&self) -> Item {
        self.item
    }
    
    pub fn io(&self) -> f32 {
        self.io
    }
    
    pub fn add_to_io(&mut self, rhs: f32) {
        self.io += rhs
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MultipleItemsIO {
    values: Vec<AveragedItemIO>,
}

impl MultipleItemsIO {
    pub fn empty() -> Self {
        MultipleItemsIO { values: vec![] }
    }
    
    pub fn add(&mut self, rhs: AveragedItemIO) {
        for item in &mut self.values {
            if item.item == rhs.item {
                item.add_to_io(rhs.io);
                break;
            }
        }
        self.values.push(rhs)
    }
}