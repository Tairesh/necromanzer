#![allow(dead_code)]

use game::actions::action::owner;
use game::avatar::Soul;
use game::map::item::{Item, ItemInteract, ItemTag};
use game::map::passage::Passage;
use game::map::terrain::{Terrain, TerrainInteract, TerrainView};
use game::World;
use geometry::direction::Direction;

pub enum ActionPossibility {
    Yes,
    No(String),
}

use self::ActionPossibility::{No, Yes};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
    Wielding(Direction),
    Dropping(usize, Direction),
    Digging(Direction),
    Reading(Direction),
    Animate(Direction), // TODO: write test for animate
}

// TODO: get rid of all these unwraps
impl ActionType {
    pub fn length(&self, owner_id: usize, world: &World) -> u32 {
        match self {
            ActionType::SkippingTime => 1,
            ActionType::Walking(dir) => {
                // TODO: check avatar perks for calculating speed
                // TODO: add sqrt(2) for diagonal movement
                let k = match owner(owner_id, world).soul {
                    Soul::Zombie(..) => 0.75,
                    _ => 1.0,
                };
                let pos = owner(owner_id, world).pos + dir;
                match world.get_tile(pos).unwrap().terrain.passage() {
                    Passage::Passable(length) => (length * k).round() as u32,
                    Passage::Unpassable => 0,
                }
            }
            ActionType::Wielding(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                if let Some(item) = world.get_tile(pos).unwrap().items.last() {
                    item.wield_time(owner(owner_id, world)).round() as u32
                } else {
                    0
                }
            }
            ActionType::Dropping(i, dir) => {
                if let Some(item) = owner(owner_id, world).wield.get(*i) {
                    let k = if matches!(dir, Direction::Here) {
                        1.0
                    } else {
                        1.5
                    };
                    (item.drop_time(owner(owner_id, world)) * k).round() as u32
                } else {
                    0
                }
            }
            ActionType::Digging(dir) => {
                // TODO: check tool quality, avatar perks
                let pos = owner(owner_id, world).pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    return match tile.terrain {
                        Terrain::Grave(..) => 2000,
                        _ => 1000,
                    };
                }

                0
            }
            ActionType::Reading(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    if tile.is_readable() {
                        return tile.read().len() as u32;
                    }
                }

                0
            }
            ActionType::Animate(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    return tile
                        .items
                        .iter()
                        .filter(|i| matches!(i, Item::Corpse(..)))
                        .map(|i| i.mass() / 10)
                        .next()
                        .unwrap_or(0);
                }

                0
            }
        }
    }

    pub fn is_possible(&self, owner_id: usize, world: &World) -> ActionPossibility {
        match self {
            ActionType::SkippingTime => Yes,
            ActionType::Walking(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    if !tile.terrain.is_passable() {
                        return No(format!("You can't walk to the {}", tile.terrain.name()));
                    }
                    if !tile.units.is_empty() {
                        let i = tile.units.iter().copied().next().unwrap();
                        return No(format!(
                            "{} is on the way",
                            world.units.get(i).unwrap().character.name
                        ));
                    }

                    Yes
                } else {
                    No("Tile isn't loaded yet".to_string())
                }
            }
            ActionType::Wielding(dir) => {
                if !owner(owner_id, world).wield.is_empty() {
                    return No("You already have something in your hands".to_string());
                }
                let pos = owner(owner_id, world).pos + dir;
                if world.get_tile(pos).unwrap().items.is_empty() {
                    return No("There is nothing to pick up".to_string());
                }
                Yes
            }
            ActionType::Dropping(_, dir) => {
                if owner(owner_id, world).wield.is_empty() {
                    return No("You have nothing to drop".to_string());
                }
                let pos = owner(owner_id, world).pos + dir;
                let terrain = &world.get_tile(pos).unwrap().terrain;
                if !terrain.is_passable() {
                    return No(format!("You can't put items on {}", terrain.name()));
                }
                Yes
            }
            ActionType::Digging(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                let terrain = &world.get_tile(pos).unwrap().terrain;
                if !terrain.is_diggable() {
                    return No(format!("You can't dig the {}", terrain.name()));
                }
                if !owner(owner_id, world)
                    .wield
                    .iter()
                    .any(|i| i.tags().contains(&ItemTag::Dig))
                {
                    return No("You need a shovel to dig!".to_string());
                }
                Yes
            }
            ActionType::Reading(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                // TODO: check skill of reading, and probably even another languages
                if let Some(tile) = world.get_tile(pos) {
                    if tile.is_readable() {
                        return Yes;
                    }
                }

                No("There is nothing to read".to_string())
            }
            ActionType::Animate(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    if tile.items.iter().any(|i| matches!(i, Item::Corpse(..))) {
                        return Yes;
                    }
                }

                No("There is nothing to rise".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::human::body::{BodyPartData, Freshness};
    use super::super::super::human::character::Character;
    use super::super::super::human::gender::Gender;
    use super::super::super::human::main_hand::MainHand;
    use super::super::super::human::skin_tone::SkinTone;
    use super::super::super::map::item::Item;
    use super::super::super::map::items::{Axe, BodyPart, Gravestone, Shovel};
    use super::super::super::map::pos::TilePos;
    use super::super::super::map::terrain::Terrain;
    use super::super::super::map::terrains::{Dirt, Grave, GraveData, GraveVariant};
    use super::super::super::world::tests::prepare_world;
    use super::super::{Action, ActionResult, ActionType};
    use geometry::direction::{Direction, DIR8};

    #[test]
    fn test_walking() {
        // TODO: add checks for failing to move to impassable terrains and units
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Dirt::default().into();

        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert_eq!(0, world.meta.current_tick);

        let typ = ActionType::Walking(Direction::East);
        let length = typ.length(0, &world);
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        world.tick();

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(1, 0), world.player().pos);
    }

    #[test]
    fn test_wielding() {
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(1, 0)).items.clear();
        world
            .load_tile_mut(TilePos::new(1, 0))
            .items
            .push(Axe::new().into());

        assert!(world.player().wield.is_empty());
        assert_eq!(0, world.meta.current_tick);

        let typ = ActionType::Wielding(Direction::East);
        let length = typ.length(0, &world);
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        world.tick();

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert_eq!(1, world.player().wield.len());
        let item = world.player().wield.first().unwrap();
        assert!(matches!(item, Item::Axe(..)));
    }

    #[test]
    fn test_skipping_time() {
        let mut world = prepare_world();

        assert_eq!(0, world.meta.current_tick);
        let typ = ActionType::SkippingTime;
        let length = typ.length(0, &world);
        assert_eq!(1, length);
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        world.tick();
        assert_eq!(1, world.meta.current_tick);
    }

    #[test]
    fn test_dropping() {
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(0, 0)).terrain = Dirt::default().into();
        world.load_tile_mut(TilePos::new(0, 0)).items.clear();
        world.player_mut().wield.clear();
        world.player_mut().wield.push(Axe::new().into());

        let typ = ActionType::Dropping(0, Direction::Here);
        let length = typ.length(0, &world);
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        world.tick();

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert_eq!(0, world.player().wield.len());
        assert_eq!(1, world.load_tile(TilePos::new(0, 0)).items.len());
        let item = world.load_tile(TilePos::new(0, 0)).items.first().unwrap();
        assert!(matches!(item, Item::Axe(..)));
    }

    #[test]
    fn test_digging() {
        let mut world = prepare_world();
        world.player_mut().wield.clear();
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Dirt::default().into();

        let typ = ActionType::Digging(Direction::East);
        let length = typ.length(0, &world);
        assert!(Action::new(0, typ, &world).is_err());

        world.player_mut().wield.push(Shovel::new().into());
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        while world.player().action.is_some() {
            world.tick();
        }

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert!(matches!(
            world.load_tile(TilePos::new(1, 0)).terrain,
            Terrain::Pit(..)
        ));

        let character = Character::new("test", Gender::Male, 25, MainHand::Right, SkinTone::Amber);
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Grave::new(
            GraveVariant::New,
            GraveData {
                character,
                death_year: 255,
            },
        )
        .into();
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        while world.player().action.is_some() {
            world.tick();
        }
        assert!(matches!(
            world.load_tile(TilePos::new(1, 0)).terrain,
            Terrain::Pit(..)
        ));
        let mut corpse = None;
        let mut gravestone = None;
        for dir in DIR8 {
            for item in world.load_tile_mut(TilePos::new(1, 0) + dir).items.iter() {
                match item {
                    Item::Corpse(..) => {
                        corpse = Some(item.clone());
                    }
                    Item::Gravestone(..) => {
                        gravestone = Some(item.clone());
                    }
                    _ => {}
                }
            }
        }
        assert!(corpse.is_some());
        if let Some(corpse) = corpse {
            if let Item::Corpse(corpse) = corpse {
                let ch = &corpse.character;
                let body = &corpse.body;
                assert_eq!("test", ch.name);
                assert_eq!(SkinTone::Amber, ch.skin_tone);
                assert_eq!(Gender::Male, ch.gender);
                assert_eq!(25, ch.age);
                assert_eq!(MainHand::Right, ch.main_hand);
                assert!(matches!(
                    body.parts.get("torso"),
                    Some(Item::BodyPart(BodyPart {
                        data: BodyPartData {
                            freshness: Freshness::Rotten,
                            ..
                        },
                        ..
                    }))
                ));
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
        assert!(gravestone.is_some());
        if let Some(gravestone) = gravestone {
            if let Item::Gravestone(gravestone) = gravestone {
                let data = &gravestone.data;
                assert_eq!("test", data.character.name);
                assert_eq!(SkinTone::Amber, data.character.skin_tone);
                assert_eq!(Gender::Male, data.character.gender);
                assert_eq!(25, data.character.age);
                assert_eq!(MainHand::Right, data.character.main_hand);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_reading() {
        let mut world = prepare_world();

        let character = Character::new("test", Gender::Male, 25, MainHand::Right, SkinTone::Amber);
        let data = GraveData {
            character,
            death_year: 255,
        };
        world.load_tile_mut(TilePos::new(1, 0)).terrain =
            Grave::new(GraveVariant::New, data.clone()).into();
        let typ = ActionType::Reading(Direction::East);
        let length = typ.length(0, &world);
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        while world.player().action.is_some() {
            let results = world.tick();
            for result in results {
                match result {
                    ActionResult::LogMessage(s) => {
                        assert_eq!("You read on gravestone: test. 230 — 255", s);
                    }
                    _ => {}
                }
            }
        }
        assert_eq!(length as u128, world.meta.current_tick);

        world.load_tile_mut(TilePos::new(0, 1)).terrain = Dirt::default().into();
        world.load_tile_mut(TilePos::new(0, 1)).items.clear();
        let typ = ActionType::Reading(Direction::South);
        assert!(Action::new(0, typ, &world).is_err());

        world
            .load_tile_mut(TilePos::new(0, 1))
            .items
            .push(Gravestone::new(data).into());

        let length = typ.length(0, &world);
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        while world.player().action.is_some() {
            let results = world.tick();
            for result in results {
                match result {
                    ActionResult::LogMessage(s) => {
                        assert_eq!("You read on gravestone: test. 230 — 255", s);
                    }
                    _ => {}
                }
            }
        }
        assert_eq!(length as u128 * 2, world.meta.current_tick);
    }
}
