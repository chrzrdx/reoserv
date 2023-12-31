use eolib::data::{EoSerialize, EoWriter};
use eolib::protocol::net::server::{
    WelcomeCode, WelcomeReplyServerPacket, WelcomeReplyServerPacketWelcomeCodeData,
};
use eolib::protocol::net::{PacketAction, PacketFamily};
use eolib::protocol::Coords;

use crate::character::Character;
use crate::errors::DataNotFoundError;
use crate::player::ClientState;
use crate::SETTINGS;

use super::super::World;

impl World {
    pub async fn select_character(&mut self, player_id: i32, character_id: i32) {
        let player = match self.players.get(&player_id) {
            Some(player) => player,
            None => return,
        };

        let account_id = match player.get_account_id().await {
            Ok(account_id) => account_id,
            Err(e) => {
                player.close(format!("Error getting account id: {}", e));
                return;
            }
        };

        let mut conn = match self.pool.get_conn().await {
            Ok(conn) => conn,
            Err(e) => {
                player.close(format!("Error getting connection from pool: {}", e));
                return;
            }
        };

        let mut character = match Character::load(&mut conn, character_id).await {
            Ok(character) => character,
            Err(_) => {
                player.close(format!(
                    "Tried to select character that doesn't exist: {}",
                    character_id
                ));
                return;
            }
        };

        if character.account_id != account_id {
            player.close(format!(
                "Player {} attempted to login to character ({}) belonging to another account: {}",
                account_id, character.name, character.account_id
            ));
            return;
        }

        character.player_id = Some(player_id);
        character.player = Some(player.clone());
        character.logged_in_at = Some(chrono::Utc::now());

        character.calculate_stats();

        if let Some(maps) = self.maps.as_ref() {
            if !maps.contains_key(&character.map_id) {
                if maps.contains_key(&SETTINGS.rescue.map) {
                    character.map_id = SETTINGS.rescue.map;
                    character.coords = Coords {
                        x: SETTINGS.rescue.x,
                        y: SETTINGS.rescue.y,
                    };
                } else {
                    player.close(format!(
                        "Rescue map not found! {}",
                        DataNotFoundError::new("map".to_string(), SETTINGS.rescue.map,)
                    ));
                    return;
                }
            }
        }

        let select_character = match self
            .get_welcome_request_data(player.clone(), &character)
            .await
        {
            Ok(select_character) => select_character,
            Err(e) => {
                player.close(format!("Error getting welcome request data: {}", e));
                return;
            }
        };

        self.characters
            .insert(character.name.to_string(), player_id);

        player.set_character(Box::new(character));
        player.set_state(ClientState::EnteringGame);

        let reply = WelcomeReplyServerPacket {
            welcome_code: WelcomeCode::SelectCharacter,
            welcome_code_data: Some(WelcomeReplyServerPacketWelcomeCodeData::SelectCharacter(
                select_character,
            )),
        };

        let mut writer = EoWriter::new();
        reply.serialize(&mut writer);
        player.send(
            PacketAction::Reply,
            PacketFamily::Welcome,
            writer.to_byte_array(),
        );
    }
}
