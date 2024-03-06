// SPDX-License-Identifier: MIT
use alsactl::{prelude::*, *};
use glib::Error;

fn elem_iface_type_to_str(elem_iface_type: &ElemIfaceType) -> &str {
    match elem_iface_type {
        ElemIfaceType::Card => "Card",
        ElemIfaceType::Hwdep => "Hwdep",
        ElemIfaceType::Mixer => "Mixer",
        ElemIfaceType::Pcm => "Pcm",
        ElemIfaceType::Rawmidi => "Rawmidi",
        ElemIfaceType::Timer => "Timer",
        ElemIfaceType::Sequencer => "Sequencer",
        _ => "Unknown",
    }
}

fn elem_type_to_str(elem_type: &ElemType) -> &str {
    match elem_type {
        ElemType::None => "None",
        ElemType::Boolean => "Boolean",
        ElemType::Integer => "Integer",
        ElemType::Enumerated => "Enumerated",
        ElemType::Bytes => "Bytes",
        ElemType::Iec60958 => "Iec60958",
        ElemType::Integer64 => "Integer64",
        _ => "Unknown",
    }
}

fn dump_elem_data(card: &alsactl::Card, elem_id: &alsactl::ElemId) -> Result<(), Error> {
    println!("Element {}:", elem_id.numid());
    println!("  device_id:      {}", elem_id.device_id());
    println!("  subdevice_id:   {}", elem_id.subdevice_id());
    println!("  name :          {}", elem_id.name());
    println!(
        "  iface:          {}",
        elem_iface_type_to_str(&elem_id.iface())
    );
    println!("  index:          {}", elem_id.index());

    let elem_info = card.elem_info(elem_id)?;

    let (elem_access, elem_owner, elem_type) = match &elem_info {
        ElemInfo::Iec60958(info) => (info.access(), info.owner(), info.elem_type()),
        ElemInfo::Boolean(info) => (info.access(), info.owner(), info.elem_type()),
        ElemInfo::Bytes(info) => (info.access(), info.owner(), info.elem_type()),
        ElemInfo::Integer(info) => (info.access(), info.owner(), info.elem_type()),
        ElemInfo::Integer64(info) => (info.access(), info.owner(), info.elem_type()),
        ElemInfo::Enumerated(info) => (info.access(), info.owner(), info.elem_type()),
    };
    println!("  access:         {:?}", elem_access);
    println!("  owner:          {}", elem_owner);
    println!("  type:           {}", elem_type_to_str(&elem_type));

    let value_count = match &elem_info {
        ElemInfo::Iec60958(_) => 0,
        ElemInfo::Boolean(info) => info.value_count(),
        ElemInfo::Bytes(info) => info.value_count(),
        ElemInfo::Integer(info) => info.value_count(),
        ElemInfo::Integer64(info) => info.value_count(),
        ElemInfo::Enumerated(info) => info.value_count(),
    };
    if value_count > 0 {
        println!("  value-count:    {}", value_count);
    }

    match &elem_info {
        ElemInfo::Integer(info) => {
            println!("    min:          {}", info.value_min());
            println!("    max:          {}", info.value_max());
            println!("    step:         {}", info.value_step());
        }
        ElemInfo::Integer64(info) => {
            println!("    min:          {}", info.value_min());
            println!("    max:          {}", info.value_max());
            println!("    step:         {}", info.value_step());
        }
        ElemInfo::Enumerated(info) => {
            info.labels().iter().enumerate().for_each(|(i, label)| {
                println!("    {}:       {}", i, label);
            });
        }
        _ => (),
    }

    let mut elem_value = ElemValue::new();
    card.read_elem_value(elem_id, &mut elem_value)?;

    match &elem_info {
        ElemInfo::Iec60958(_) => {
            println!(
                "  channel_status: {:?}",
                elem_value.iec60958_channel_status()
            );
            println!("  user_data:      {:?}", elem_value.iec60958_user_data());
        }
        ElemInfo::Boolean(info) => {
            let value_count = info.value_count() as usize;
            println!(
                "  values:         {:?}",
                &elem_value.boolean()[..value_count]
            );
        }
        ElemInfo::Bytes(info) => {
            let value_count = info.value_count() as usize;
            println!("  values:         {:?}", &elem_value.bytes()[..value_count]);
        }
        ElemInfo::Integer(info) => {
            let value_count = info.value_count() as usize;
            println!("  values:         {:?}", &elem_value.int()[..value_count]);
        }
        ElemInfo::Integer64(info) => {
            let value_count = info.value_count() as usize;
            println!("  values:         {:?}", &elem_value.int64()[..value_count]);
        }
        ElemInfo::Enumerated(info) => {
            let labels = info.labels();
            let value_count = info.value_count() as usize;
            let vals: Vec<_> = elem_value
                .enumerated()
                .iter()
                .take(value_count)
                .map(|&val| &labels[val as usize])
                .collect();
            println!("  values:         {:?}", vals);
        }
    }

    if elem_access.contains(ElemAccessFlag::TLV_READ) {
        let mut cntr = vec![0; 64];
        card.read_elem_tlv(elem_id, &mut cntr)?;
        println!("  tlv:            {:?}", cntr);
    }

    Ok(())
}

fn main() {
    let card_id_list = match alsactl::card_id_list() {
        Ok(entries) => entries,
        Err(_) => {
            eprintln!("Fail to get the list of sound card.");
            std::process::exit(1);
        }
    };

    card_id_list.iter().for_each(|&card_id| {
        let card = Card::new();
        if card.open(card_id, 0).is_err() {
            eprintln!("Fail to open sound card: {}", card_id);
            std::process::exit(1);
        }

        let elem_id_list = match card.elem_id_list() {
            Ok(entries) => entries,
            Err(_) => {
                eprintln!("Fail to get the list of element for sound card {}", card_id);
                std::process::exit(1);
            }
        };

        elem_id_list.iter().for_each(|elem_id| {
            if dump_elem_data(&card, elem_id).is_err() {
                eprintln!("Fail to dump the data of element: {}", elem_id.name());
                std::process::exit(1);
            }
        });
    });
}
