use base::*;

use crate::base::manual_definition::items::Item;

mod base;
mod save_manager;

fn main() {
    let item = Item::IronIngot;
    println!("{}", item.get_name())
}
