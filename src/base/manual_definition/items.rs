use super::recipes::RecipeEnum;
use super::sources::SourceEnum;

#[derive(Clone, PartialEq, Debug)]
pub struct Item {
    variant: ItemEnum,
    name: String,
    recipes: Vec<RecipeEnum>,
    source: Option<SourceEnum>,
    fluid: bool,
}

impl Item {
    pub fn new_fluid(name: &str, recipes: Vec<RecipeEnum>, source: Option<SourceEnum>, variant: ItemEnum) -> Self {
        Item { name: name.to_owned(), recipes, source, fluid: true, variant}
    }
    pub fn new_solid(name: &str, recipes: Vec<RecipeEnum>, source: Option<SourceEnum>, variant: ItemEnum) -> Self {
        Item { name: name.to_owned(), recipes, source, fluid: false, variant}
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum ItemEnum {
    IronOre,
    IronIngot,
}

impl ItemEnum {
    pub fn get_item(&self) -> Item {
        match self {
            Self::IronOre => {
                let name = "Iron Ore";
                let recipes = vec![];
                let source = Some(SourceEnum::Mine(60.));
                
                Item::new_solid(name, recipes, source, *self)
            },
            
            Self::IronIngot => {
                let name = "Iron Ingot";
                let recipes = vec![RecipeEnum::Iron2Ingot];
                let source = None;
                
                Item::new_solid(name, recipes, source, *self)
            },
        }
    }
}

/// Items used in a recipe for example, always an integer
#[derive(Clone, PartialEq, Debug)]
pub struct RawItemIO {
    item: ItemEnum,
    quantity: u32,
}

impl RawItemIO {
    pub fn new(item: ItemEnum, quantity: u32) -> Self {
        RawItemIO { item, quantity }
    }
}

/// Items produced or used per minute
#[derive(Clone, PartialEq, Debug)]
pub struct AveragedItemIO {
    item: ItemEnum,
    quantity: f32,
}

impl AveragedItemIO {
    pub fn new(item: ItemEnum, quantity: f32) -> Self {
        AveragedItemIO { item, quantity }
    }
}