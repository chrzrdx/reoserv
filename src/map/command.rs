use bytes::Bytes;
use eo::{
    data::{i32, EOInt, EOShort, EOThree},
    protocol::{
        server::range, Coords, Direction, Emote, Item, NearbyInfo, ShortItem, StatId, WarpAnimation,
    },
};
use tokio::sync::oneshot;

use crate::{
    character::{Character, SpellTarget},
    player::PartyRequest,
};

#[derive(Debug)]
pub enum Command {
    AcceptTrade {
        player_id: EOShort,
    },
    AcceptTradeRequest {
        player_id: EOShort,
        target_player_id: EOShort,
    },
    AddChestItem {
        player_id: EOShort,
        item: Item,
    },
    AddLockerItem {
        player_id: EOShort,
        item: Item,
    },
    AddTradeItem {
        player_id: EOShort,
        item: Item,
    },
    Attack {
        target_player_id: EOShort,
        direction: Direction,
        timestamp: EOThree,
    },
    BuyItem {
        player_id: EOShort,
        item: Item,
        session_id: EOShort,
    },
    CancelTrade {
        player_id: EOShort,
        partner_player_id: EOShort,
    },
    CastSpell {
        player_id: EOShort,
        target: SpellTarget,
    },
    CraftItem {
        player_id: EOShort,
        item_id: EOShort,
        session_id: EOShort,
    },
    CreateBoardPost {
        player_id: EOShort,
        subject: String,
        body: String,
    },
    DepositGold {
        player_id: EOShort,
        session_id: EOThree,
        amount: EOInt,
    },
    DropItem {
        target_player_id: EOShort,
        item: ShortItem,
        coords: Coords,
    },
    Emote {
        target_player_id: EOShort,
        emote: Emote,
    },
    Enter {
        character: Box<Character>,
        warp_animation: Option<WarpAnimation>,
        respond_to: oneshot::Sender<()>,
    },
    Equip {
        player_id: EOShort,
        item_id: EOShort,
        sub_loc: i32,
    },
    Face {
        target_player_id: EOShort,
        direction: Direction,
    },
    ForgetSkill {
        player_id: EOShort,
        skill_id: EOShort,
        session_id: EOShort,
    },
    GetCharacter {
        player_id: EOShort,
        respond_to: oneshot::Sender<Option<Box<Character>>>,
    },
    GetDimensions {
        respond_to: oneshot::Sender<(i32, i32)>,
    },
    GetItem {
        target_player_id: EOShort,
        item_index: EOShort,
    },
    GetMapInfo {
        player_ids: Vec<EOShort>,
        npc_indexes: Vec<i32>,
        respond_to: oneshot::Sender<range::Reply>,
    },
    GetNearbyInfo {
        target_player_id: EOShort,
        respond_to: oneshot::Sender<NearbyInfo>,
    },
    GetRelogCoords {
        respond_to: oneshot::Sender<Option<Coords>>,
    },
    GetRidAndSize {
        respond_to: oneshot::Sender<([EOShort; 2], EOInt)>,
    },
    GiveItem {
        target_player_id: EOShort,
        item_id: EOShort,
        amount: EOInt,
    },
    HasPlayer {
        player_id: EOShort,
        respond_to: oneshot::Sender<bool>,
    },
    JunkItem {
        target_player_id: EOShort,
        item_id: EOShort,
        amount: EOInt,
    },
    LearnSkill {
        player_id: EOShort,
        spell_id: EOShort,
        session_id: EOShort,
    },
    Leave {
        player_id: EOShort,
        warp_animation: Option<WarpAnimation>,
        interact_player_id: Option<EOShort>,
        respond_to: oneshot::Sender<Character>,
    },
    LevelStat {
        player_id: EOShort,
        stat_id: StatId,
    },
    OpenBank {
        player_id: EOShort,
        npc_index: i32,
    },
    OpenBoard {
        player_id: EOShort,
        board_id: EOShort,
    },
    OpenChest {
        player_id: EOShort,
        coords: Coords,
    },
    OpenDoor {
        target_player_id: EOShort, // TODO: rename to player_id
        door_coords: Coords,       // TODO: rename to coords
    },
    OpenInn {
        player_id: EOShort,
        npc_index: i32,
    },
    OpenLocker {
        player_id: EOShort,
    },
    OpenShop {
        player_id: EOShort,
        npc_index: i32,
    },
    OpenSkillMaster {
        player_id: EOShort,
        npc_index: i32,
    },
    RecoverNpcs,
    RecoverPlayers,
    RemoveBoardPost {
        player_id: EOShort,
        post_id: EOShort,
    },
    RemoveCitizenship {
        player_id: EOShort,
    },
    RemoveTradeItem {
        player_id: EOShort,
        item_id: EOShort,
    },
    RequestCitizenship {
        player_id: EOShort,
        session_id: EOShort,
        answers: [String; 3],
    },
    RequestPaperdoll {
        player_id: EOShort,
        target_player_id: EOShort,
    },
    RequestSleep {
        player_id: EOShort,
        session_id: EOShort,
    },
    PartyRequest {
        target_player_id: EOShort,
        request: PartyRequest,
    },
    RequestTrade {
        player_id: EOShort,
        target_player_id: EOShort,
    },
    ResetCharacter {
        player_id: EOShort,
        session_id: EOShort,
    },
    Save {
        respond_to: oneshot::Sender<()>,
    },
    SellItem {
        player_id: EOShort,
        item: Item,
        session_id: EOShort,
    },
    SendChatMessage {
        target_player_id: EOShort,
        message: String,
    },
    Serialize {
        respond_to: oneshot::Sender<Bytes>,
    },
    Sit {
        player_id: EOShort,
    },
    SitChair {
        player_id: EOShort,
        coords: Coords,
    },
    Sleep {
        player_id: EOShort,
        session_id: EOShort,
    },
    Stand {
        player_id: EOShort,
    },
    StartSpellChant {
        player_id: EOShort,
        spell_id: EOShort,
        timestamp: EOThree,
    },
    TakeChestItem {
        player_id: EOShort,
        item_id: EOShort,
    },
    TakeLockerItem {
        player_id: EOShort,
        item_id: EOShort,
    },
    TimedArena,
    TimedDoorClose,
    TimedDrain,
    TimedQuake,
    TimedSpikes,
    TimedWarpSuck,
    ToggleHidden {
        player_id: EOShort,
    },
    UnacceptTrade {
        player_id: EOShort,
    },
    Unequip {
        player_id: EOShort,
        item_id: EOShort,
        sub_loc: i32,
    },
    UpgradeLocker {
        player_id: EOShort,
    },
    UseItem {
        player_id: EOShort,
        item_id: EOShort,
    },
    ViewBoardPost {
        player_id: EOShort,
        post_id: EOShort,
    },
    Walk {
        target_player_id: EOShort,
        direction: Direction,
        coords: Coords,
        timestamp: EOThree,
    },
    WithdrawGold {
        player_id: EOShort,
        session_id: EOThree,
        amount: EOInt,
    },
    SpawnItems,
    SpawnNpcs,
    ActNpcs,
}
