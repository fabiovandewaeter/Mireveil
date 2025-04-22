use crate::entities::entity::Entity;

pub enum ItemKind

pub struct Item {
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub kind: ItemKind,
}

/*
pub enum ItemKind {
    Sword,
    Helmet,
}*/

pub trait Equipable {
    fn equip(&self, entity: &mut Entity);
    fn unequip(&self, entity: &mut Entity);
    /// returns the EquipmentSlot the Item can be equipped on
    fn get_slot(&self) -> EquipmentSlot;
}

pub trait Usable {
    fn use_on(&self, entity: &mut Entity);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Head,
    Chest,
    Legs,
    Feet,
}

pub struct Sword {
    pub damage: u32,
}

pub struct Helmet {
    pub defense: u32,
}
pub struct Chestplate {
    pub defense: u32,
}
pub struct Leggings {
    pub defense: u32,
}
pub struct Boots {
    pub defense: u32,
}

pub struct HealthPotion {
    pub healing: u32,
}
