use eo::data::{EOInt, EOShort};

use crate::SETTINGS;

use super::Map;

impl Map {
    pub fn give_experience(
        &mut self,
        player_id: EOShort,
        experience: EOInt,
    ) -> (bool, EOInt, EOInt) {
        match self.characters.get_mut(&player_id) {
            Some(character) => {
                let experience = experience * SETTINGS.world.exp_multiplier;
                let leveled_up = character.add_experience(experience);
                (leveled_up, character.experience, experience)
            }
            None => (false, 0, 0),
        }
    }
}
