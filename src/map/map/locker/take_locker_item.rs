use std::cmp;

use eolib::{
    data::{EoSerialize, EoWriter},
    protocol::{
        map::MapTileSpec,
        net::{server::LockerGetServerPacket, PacketAction, PacketFamily, ThreeItem},
        Coords,
    },
};

use super::super::Map;

impl Map {
    pub fn take_locker_item(&mut self, player_id: i32, item_id: i32) {
        let character = match self.characters.get(&player_id) {
            Some(character) => character,
            None => return,
        };

        let adjacent_tiles = [
            self.get_tile(&Coords {
                x: character.coords.x,
                y: character.coords.y - 1,
            }),
            self.get_tile(&Coords {
                x: character.coords.x,
                y: character.coords.y + 1,
            }),
            self.get_tile(&Coords {
                x: character.coords.x - 1,
                y: character.coords.y,
            }),
            self.get_tile(&Coords {
                x: character.coords.x + 1,
                y: character.coords.y,
            }),
        ];

        if !adjacent_tiles.iter().any(|tile| match tile {
            Some(tile) => *tile == MapTileSpec::BankVault,
            None => false,
        }) {
            return;
        }

        let amount = character.get_bank_item_amount(item_id);
        if amount == 0 {
            return;
        }

        let amount = cmp::min(character.can_hold(item_id, amount), amount);
        if amount == 0 {
            return;
        }

        let character = match self.characters.get_mut(&player_id) {
            Some(character) => character,
            None => return,
        };

        character.remove_bank_item(item_id, amount);
        character.add_item(item_id, amount);

        let packet = LockerGetServerPacket {
            taken_item: ThreeItem {
                id: item_id,
                amount: character.get_item_amount(item_id),
            },
            weight: character.get_weight(),
            locker_items: character
                .bank
                .iter()
                .map(|i| ThreeItem {
                    id: i.id,
                    amount: i.amount,
                })
                .collect(),
        };

        let mut writer = EoWriter::new();

        if let Err(e) = packet.serialize(&mut writer) {
            error!("Failed to serialize LockerGetServerPacket: {}", e);
            return;
        }

        character.player.as_ref().unwrap().send(
            PacketAction::Get,
            PacketFamily::Locker,
            writer.to_byte_array(),
        );
    }
}
