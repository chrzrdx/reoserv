use std::collections::HashMap;

use eo::{
    data::{EOShort, Serializeable},
    protocol::{server::talk, PacketAction, PacketFamily},
};

use crate::player::{PlayerHandle, ClientState};

pub async fn broadcast_global_message(
    target_player_id: EOShort,
    name: &str,
    message: &str,
    players: &HashMap<EOShort, PlayerHandle>,
) {
    let packet = talk::Msg {
        player_name: name.to_string(),
        message: message.to_string(),
    };
    let buf = packet.serialize();
    for player in players.values() {
        let state = player.get_state().await;
        let player_id = player.get_player_id().await;
        if state == ClientState::Playing && player_id != target_player_id {
            player.send(PacketAction::Msg, PacketFamily::Talk, buf.clone());
        }
    }
}
