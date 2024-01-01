use eolib::{
    data::{EoSerialize, EoWriter},
    protocol::{
        net::{server::BankOpenServerPacket, PacketAction, PacketFamily},
        r#pub::NpcType,
    },
};

use crate::NPC_DB;

use super::super::Map;

impl Map {
    pub async fn open_bank(&mut self, player_id: i32, npc_index: i32) {
        let npc = match self.npcs.get(&npc_index) {
            Some(npc) => npc,
            None => return,
        };

        let npc_data = match NPC_DB.npcs.get(npc.id as usize - 1) {
            Some(npc_data) => npc_data,
            None => return,
        };

        if npc_data.r#type != NpcType::Bank {
            return;
        }

        let character = match self.characters.get(&player_id) {
            Some(character) => character,
            None => return,
        };

        let player = match character.player.as_ref() {
            Some(player) => player,
            None => return,
        };

        let session_id = match player.generate_session_id().await {
            Ok(id) => id,
            Err(e) => {
                error!("Failed to generate session id {}", e);
                return;
            }
        };

        player.set_interact_npc_index(npc_index);

        let open = BankOpenServerPacket {
            gold_bank: character.gold_bank,
            session_id,
            locker_upgrades: character.bank_level,
        };

        let mut writer = EoWriter::new();

        if let Err(e) = open.serialize(&mut writer) {
            error!("Failed to serialize BankOpenServerPacket: {}", e);
            return;
        }

        player.send(
            PacketAction::Open,
            PacketFamily::Bank,
            writer.to_byte_array(),
        );
    }
}
