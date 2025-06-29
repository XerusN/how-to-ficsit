use super::recipes::RecipeEnum;
use super::sources::SourceEnum;

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq, Copy)]
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