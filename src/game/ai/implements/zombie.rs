use game::actions::ActionType;
use game::ai::brain::Brain;
use geometry::direction::Direction;
use rand::{thread_rng, Rng};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ZombieBrain {
    action: ActionType,
}

impl ZombieBrain {
    pub fn new() -> Self {
        Self {
            action: ActionType::SkippingTime,
        }
    }
}

impl Default for ZombieBrain {
    fn default() -> Self {
        Self::new()
    }
}

impl Brain for ZombieBrain {
    fn plan(&mut self) {
        // TODO: use world.rng
        let mut rng = thread_rng();

        self.action = ActionType::Walking(match rng.gen_range(0..5) {
            0 => Direction::East,
            1 => Direction::West,
            2 => Direction::North,
            3 => Direction::South,
            4 => Direction::Here,
            _ => unreachable!(),
        });
    }

    fn action(&self) -> Option<ActionType> {
        Some(self.action)
    }
}
