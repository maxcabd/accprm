use super::calc_crc32;
use std::{collections::HashMap, path::Path};

use crate::cfg::Config;
use nuccbin::{
    nucc_binary::{AccessoriesParam, AccessoryParam, MessageInfo, NuccBinaryParsed},
    NuccBinaryType,
};

pub fn add_message_info_entry(
    nucc_binaries: &mut HashMap<NuccBinaryType, Box<dyn NuccBinaryParsed>>,
    cfg: &Config,
) {
    let message_info = nucc_binaries
        .get_mut(&NuccBinaryType::MessageInfo)
        .unwrap()
        .downcast_mut::<MessageInfo>()
        .unwrap();

    let mut entries = Vec::new();

    for accessory in cfg.accessories.iter() {
        let accessory_name_exists = message_info.entries.iter().any(|entry| {
            entry.text3 == accessory.accessory_name
                && entry.crc32 == calc_crc32(&accessory.accessory_id)
        });

        let base_entry = message_info
            .entries
            .iter_mut()
            .find(|entry| entry.crc32 == [246, 160, 24, 181]) // Some random crc32 value that exists
            .unwrap();

        let mut entry = base_entry.clone();
        entry.crc32 = calc_crc32(&accessory.accessory_id);
        entry.text3 = accessory.accessory_name.clone();

        if accessory_name_exists {
            continue;
        }
        entries.push(entry);
    }

    message_info.entries.extend(entries);
}

pub fn add_accessories_entry(
    nucc_binaries: &mut HashMap<NuccBinaryType, Box<dyn NuccBinaryParsed>>,
    cfg: &Config,
) {
    let accessories_param = nucc_binaries
        .get_mut(&NuccBinaryType::AccessoriesParam)
        .unwrap()
        .downcast_mut::<AccessoriesParam>()
        .unwrap();

    let mut entries = Vec::new();

    let mut highest_acc_link = accessories_param
        .entries
        .iter()
        .map(|entry| {
            entry
                .accessory_link
                .split("_")
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .max()
        .unwrap_or(0)
        + 10;

    for accessory in cfg.accessories.iter() {
        let accessory_exists = accessories_param
            .entries
            .iter()
            .any(|entry| entry.accessory_name_id == accessory.accessory_id);

        let latest_entry = accessories_param.entries.last().unwrap();

        let mut entry = latest_entry.clone();

        entry.price = 100;
        entry.unlock_condition = 0;

        entry.accessory_name_id = accessory.accessory_id.clone();
        entry.accessory_link = format!("Acce_{}", highest_acc_link);
        entry.accessory = accessory.modelcode.clone();

        entry.icon = Path::new(&accessory.icon_filepath)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".png", ".xfbin");

        if accessory_exists {
            continue;
        }

        entries.push(entry);

        highest_acc_link += 10;
    }

    accessories_param.entries.extend(entries);
}

pub fn add_accessory_entry(
    nucc_binaries: &mut HashMap<NuccBinaryType, Box<dyn NuccBinaryParsed>>,
    cfg: &Config,
) {
    let accessory_param = nucc_binaries
        .get_mut(&NuccBinaryType::AccessoryParam)
        .and_then(|param| param.downcast_mut::<AccessoryParam>())
        .expect("Failed to retrieve accessoryParam");

    let mut entries = Vec::new();

    for accessory in cfg.accessories.iter() {
        let accessory_exists = accessory_param
            .entries
            .iter()
            .any(|entry| entry.accessory == accessory.modelcode);

        let latest_entry = accessory_param.entries.last().unwrap();

        let mut entry = latest_entry.clone();

        // TODO: Figure out what unk1 does later
        entry.head_a = accessory.head_a as u32;
        entry.head_b = accessory.head_b as u32;
        entry.face = accessory.face as u32;
        entry.eyes = accessory.eyes as u32;
        entry.back = accessory.back as u32;
        entry.back_pocket = accessory.back_pocket as u32;
        entry.tail = accessory.tail as u32;
        entry.arms = accessory.arms as u32;

        entry.accessory = accessory.modelcode.clone();

        if accessory_exists {
            continue;
        }

        entries.push(entry);
    }

    accessory_param.entries.extend(entries);
}
