use crate::entities::entity::Entity;

pub trait Equipable {
    /// returns the EquipmentSlot the Item can be equipped on
    fn get_slot(&self) -> EquipmentSlot;
}

pub trait Usable {
    fn use_on(&self, entity: &mut Entity);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    Hand,
    Head,
    Chest,
    Legs,
    Feet,
}

#[derive(Clone, Copy)]
pub enum WeaponKind {
    Sword,
    Bow,
    Axe,
    Dagger,
    Staff,
}

#[derive(Clone, Copy)]
pub enum ArmorKind {
    Helmet,
    Chestplate,
    Boots,
}

#[derive(Clone, Copy)]
pub enum ConsumableKind {
    Potion,
    Scroll,
    Food,
}

#[derive(Clone, Copy)]
pub enum ConsumableEffect {
    Heal(u32),
}

#[derive(Clone, Copy)]
pub struct WeaponData {
    kind: WeaponKind,
    pub strenght: u32,
}

#[derive(Clone, Copy)]
pub struct ArmorData {
    kind: ArmorKind,
    defense: u32,
}

#[derive(Clone, Copy)]
pub struct ConsumableData {
    kind: ConsumableKind,
    effect: ConsumableEffect,
    charges: u32,
}

#[derive(Clone, Copy)]
pub enum ItemKind {
    Weapon(WeaponData),
    Armor(ArmorData),
    Consumable(ConsumableData),
}

pub struct Item {
    pub name: String,
    pub description: String,
    pub kind: ItemKind,
}

impl Item {
    pub fn new(name: String, description: String, kind: ItemKind) -> Self {
        Self {
            name,
            description,
            kind,
        }
    }

    pub fn new_weapon(name: String, description: String, kind: WeaponKind, strenght: u32) -> Self {
        Self {
            name,
            description,
            kind: ItemKind::Weapon(WeaponData { kind, strenght }),
        }
    }

    pub fn new_armor(name: String, description: String, kind: ArmorKind, defense: u32) -> Self {
        Self {
            name,
            description,
            kind: ItemKind::Armor(ArmorData { kind, defense }),
        }
    }

    pub fn new_consumable(
        name: String,
        description: String,
        kind: ConsumableKind,
        effect: ConsumableEffect,
        charges: u32,
    ) -> Self {
        Self {
            name,
            description,
            kind: ItemKind::Consumable(ConsumableData {
                kind,
                effect,
                charges,
            }),
        }
    }

    pub fn get_equipment_slot(&self) -> Option<EquipmentSlot> {
        match &self.kind {
            ItemKind::Weapon(_) => Some(EquipmentSlot::Hand),
            ItemKind::Armor(armor_data) => match armor_data.kind {
                ArmorKind::Helmet => Some(EquipmentSlot::Head),
                ArmorKind::Chestplate => Some(EquipmentSlot::Chest),
                ArmorKind::Boots => Some(EquipmentSlot::Feet),
            },
            ItemKind::Consumable(_) => None,
        }
    }
}

impl Usable for Item {
    fn use_on(&self, entity: &mut Entity) {
        match &self.kind {
            ItemKind::Consumable(consumable_data) => {
                match consumable_data.effect {
                    ConsumableEffect::Heal(amount) => {
                        entity.stats.hp = (entity.stats.hp + amount).min(entity.stats.max_hp);
                        println!(
                            "{} uses {} and recovers {} HP",
                            entity.name, self.name, amount
                        );
                    } // other effects...
                }
            }
            _ => println!("{} is not a consumable", self.name),
        }
    }
}
