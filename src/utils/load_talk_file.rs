use std::{fs::File, io::Read};

use bytes::Bytes;
use eo::{
    data::{i32, Serializeable, StreamReader},
    pubs::{TalkFile, TalkNpc},
};
use glob::glob;
use serde_json::Value;

use crate::SETTINGS;

use super::save_pub_file;

pub fn load_talk_file() -> Result<TalkFile, Box<dyn std::error::Error>> {
    if SETTINGS.server.generate_pub {
        load_json()
    } else {
        load_pub()
    }
}

fn load_json() -> Result<TalkFile, Box<dyn std::error::Error>> {
    let mut talk_file = TalkFile::default();
    talk_file.magic = "ETF".to_string();

    let mut npc_id = 1;
    for entry in glob("pub/npcs/*.json")? {
        let path = entry?;
        let mut file = File::open(path)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;

        let v: Value = serde_json::from_str(&json)?;

        let messages = v["talkMessages"].as_array().unwrap();
        if messages.len() > 0 {
            talk_file.npcs.push(TalkNpc {
                npc_id,
                rate: v["talkRate"].as_u64().unwrap_or(0) as i32,
                num_messages: messages.len() as i32,
                messages: messages
                    .iter()
                    .map(|v| v.as_str().unwrap_or_default().to_string())
                    .collect(),
            });
        }

        npc_id += 1;
    }

    save_pub_file(&talk_file, "pub/ttd001.etf")?;

    Ok(talk_file)
}

fn load_pub() -> Result<TalkFile, Box<dyn std::error::Error>> {
    let mut file = File::open("pub/ttd001.etf")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let bytes = Bytes::from(buf);
    let reader = StreamReader::new(bytes);

    let mut talk_file = TalkFile::default();
    talk_file.deserialize(&reader);

    Ok(talk_file)
}
