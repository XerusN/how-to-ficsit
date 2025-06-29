
use super::recipes::RecipeEnum;

#[derive(Clone, PartialEq, Debug)]
pub struct Item {
    variant: ItemEnum,
    name: String,
    recipes: Vec<RecipeEnum>,
    fluid: bool,
}

impl Item {
    pub fn new_fluid(name: &str, recipes: Vec<RecipeEnum>, variant: ItemEnum) -> Self {
        Item { name: name.to_owned(), recipes, fluid: true, variant}
    }
    pub fn new_solid(name: &str, recipes: Vec<RecipeEnum>, variant: ItemEnum) -> Self {
        Item { name: name.to_owned(), recipes, fluid: false, variant}
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
                
                Item::new_solid(name, recipes, *self)
            },
            
            Self::IronIngot => {
                let name = "Iron Ingot";
                let recipes = vec![RecipeEnum::Iron2Ingot];
                
                Item::new_solid(name, recipes, *self)
            },
        }
    }
}

/// Items used in a recipe for example, always an integer
#[derive(Clone, PartialEq, Debug, Copy)]
pub struct RawItemIO {
    item: ItemEnum,
    io: u32,
}

impl RawItemIO {
    pub fn new(item: ItemEnum, io: u32) -> Self {
        RawItemIO { item, io }
    }
}

/// Items produced or used per minute
#[derive(Clone, PartialEq, Debug, Copy)]
pub struct AveragedItemIO {
    item: ItemEnum,
    io: f32,
}

impl AveragedItemIO {
    pub fn new(item: ItemEnum, io: f32) -> Self {
        AveragedItemIO { item, io }
    }
    
    pub fn item(&self) -> ItemEnum {
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