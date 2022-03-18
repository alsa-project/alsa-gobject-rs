// SPDX-License-Identifier: MIT
use alsactl::{traits::*, *};
use glib::Error;

fn dump_elem_data(card: &alsactl::Card, elem_id: &alsactl::ElemId) -> Result<(), Error> {
    println!("Element {}:", elem_id.numid());
    println!("  device_id:      {}", elem_id.device_id());
    println!("  subdevice_id:   {}", elem_id.subdevice_id());
    println!("  name :          {}", elem_id.name());
    println!("  iface:          {}", elem_id.iface());
    println!("  index:          {}", elem_id.index());

    let elem_info = card.elem_info(elem_id)?;

    let elem_access = elem_info.access();
    let elem_type = elem_info.type_();
    let value_count = elem_info.value_count() as usize;
    println!("  access:         {:?}", elem_access);
    println!("  owner:          {}", elem_info.owner());
    println!("  value-count:    {}", value_count);
    println!("  type:           {}", elem_type);

    match elem_type {
        ElemType::Integer => {
            let data = elem_info.get_int_data()?;
            println!("    min:          {}", data[0]);
            println!("    max:          {}", data[1]);
            println!("    step:         {}", data[2]);
        }
        ElemType::Enumerated => {
            let data = elem_info.enum_data()?;
            data.iter().enumerate().for_each(|(i, label)| {
                println!("    {}:       {}", i, label);
            });
        }
        ElemType::Integer64 => {
            let data = elem_info.get_int64_data()?;
            println!("    min:          {}", data[0]);
            println!("    max:          {}", data[1]);
            println!("    step:         {}", data[2]);
        }
        _ => (),
    }

    let mut elem_value = ElemValue::new();
    card.read_elem_value(elem_id, &mut elem_value)?;

    match elem_type {
        ElemType::Boolean => {
            let mut vals = vec![false; value_count];
            elem_value.get_bool(&mut vals);
            println!("  values:         {:?}", vals);
        }
        ElemType::Integer => {
            let mut vals = vec![0; value_count];
            elem_value.get_int(&mut vals);
            println!("  values:         {:?}", vals);
        }
        ElemType::Enumerated => {
            let mut vals = vec![0; value_count];
            elem_value.get_enum(&mut vals);
            println!("  values:         {:?}", vals);
        }
        ElemType::Bytes => {
            let mut vals = vec![0; value_count];
            elem_value.get_bytes(&mut vals);
            println!("  values:         {:?}", vals);
        }
        ElemType::Iec60958 => {
            let mut channel_status = vec![0; 24];
            let mut user_data = vec![0; 147];
            elem_value.get_iec60958_channel_status(&mut channel_status);
            elem_value.get_iec60958_user_data(&mut user_data);
            println!("  channel_status: {:?}", channel_status);
            println!("  user_data:      {:?}", user_data);
        }
        ElemType::Integer64 => {
            let mut vals = vec![0; value_count];
            elem_value.get_int64(&mut vals);
            println!("  values:         {:?}", vals);
        }
        _ => {}
    }

    if elem_access.contains(ElemAccessFlag::TLV_READ) {
        let mut cntr = vec![0; 64];
        card.read_elem_tlv(elem_id, &mut cntr)?;
        println!("  tlv:            {:?}", cntr);
    }

    Ok(())
}

fn main() {
    let card_id_list = match alsactl::functions::card_id_list() {
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

        let elem_id_list = match card.get_elem_id_list() {
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
