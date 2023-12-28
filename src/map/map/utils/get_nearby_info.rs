use eo::{
    data::{i32, EOShort},
    protocol::NearbyInfo,
};
use tokio::sync::oneshot;

use crate::utils::in_client_range;

use super::super::Map;

impl Map {
    pub fn get_nearby_info(
        &self,
        target_player_id: EOShort,
        respond_to: oneshot::Sender<NearbyInfo>,
    ) {
        let target = self.characters.get(&target_player_id).unwrap();
        let mut nearby_items = Vec::new();
        let mut nearby_npcs = Vec::new();
        let mut nearby_characters = Vec::new();
        for (index, item) in self.items.iter() {
            if in_client_range(&target.coords, &item.coords) {
                nearby_items.push(item.to_item_map_info(*index));
            }
        }
        for (index, npc) in self.npcs.iter() {
            if npc.alive && in_client_range(&target.coords, &npc.coords) {
                nearby_npcs.push(npc.to_map_info(index));
            }
        }
        for character in self.characters.values() {
            if target_player_id == character.player_id.unwrap()
                || (!character.hidden && in_client_range(&target.coords, &character.coords))
            {
                nearby_characters.push(character.to_map_info());
            }
        }
        let _ = respond_to.send(NearbyInfo {
            num_characters: nearby_characters.len() as i32,
            items: nearby_items,
            npcs: nearby_npcs,
            characters: nearby_characters,
        });
    }
}
