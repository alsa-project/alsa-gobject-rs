// SPDX-License-Identifier: MIT
extern crate alsactl;
extern crate glib;
use alsactl::{traits::*, *};
use glib::Error;

fn dump_elem_data(card: &alsactl::Card, elem_id: &alsactl::ElemId) -> Result<(), Error> {
    println!("Element {}:", elem_id.get_numid());
    println!("  device_id:      {}", elem_id.get_device_id());
    println!("  subdevice_id:   {}", elem_id.get_subdevice_id());
    println!("  name :          {}", elem_id.get_name());
    println!("  iface:          {}", elem_id.get_iface());
    println!("  index:          {}", elem_id.get_index());

    let elem_info = card.get_elem_info(elem_id)?;

    let (elem_access, elem_owner, elem_type) = match &elem_info {
        ElemInfo::Iec60958(info) => (
            info.get_property_access(),
            info.get_property_owner(),
            info.get_property_elem_type(),
        ),
        ElemInfo::Boolean(info) => (
            info.get_property_access(),
            info.get_property_owner(),
            info.get_property_elem_type(),
        ),
        ElemInfo::Bytes(info) => (
            info.get_property_access(),
            info.get_property_owner(),
            info.get_property_elem_type(),
        ),
        ElemInfo::Integer(info) => (
            info.get_property_access(),
            info.get_property_owner(),
            info.get_property_elem_type(),
        ),
        ElemInfo::Integer64(info) => (
            info.get_property_access(),
            info.get_property_owner(),
            info.get_property_elem_type(),
        ),
        ElemInfo::Enumerated(info) => (
            info.get_property_access(),
            info.get_property_owner(),
            info.get_property_elem_type(),
        ),
    };
    println!("  access:         {:?}", elem_access);
    println!("  owner:          {}", elem_owner);
    println!("  type:           {}", elem_type);

    let value_count = match &elem_info {
        ElemInfo::Iec60958(_) => 0,
        ElemInfo::Boolean(info) => info.get_property_value_count(),
        ElemInfo::Bytes(info) => info.get_property_value_count(),
        ElemInfo::Integer(info) => info.get_property_value_count(),
        ElemInfo::Integer64(info) => info.get_property_value_count(),
        ElemInfo::Enumerated(info) => info.get_property_value_count(),
    };
    if value_count > 0 {
        println!("  value-count:    {}", value_count);
    }

    match &elem_info {
        ElemInfo::Integer(info) => {
            println!("    min:          {}", info.get_property_value_min());
            println!("    max:          {}", info.get_property_value_max());
            println!("    step:         {}", info.get_property_value_step());
        }
        ElemInfo::Integer64(info) => {
            println!("    min:          {}", info.get_property_value_min());
            println!("    max:          {}", info.get_property_value_max());
            println!("    step:         {}", info.get_property_value_step());
        }
        ElemInfo::Enumerated(info) => {
            info.get_property_labels()
                .iter()
                .enumerate()
                .for_each(|(i, label)| {
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
                elem_value.get_iec60958_channel_status()
            );
            println!(
                "  user_data:      {:?}",
                elem_value.get_iec60958_user_data()
            );
        }
        ElemInfo::Boolean(info) => {
            let value_count = info.get_property_value_count() as usize;
            println!(
                "  values:         {:?}",
                &elem_value.get_bool()[..value_count]
            );
        }
        ElemInfo::Bytes(info) => {
            let value_count = info.get_property_value_count() as usize;
            println!(
                "  values:         {:?}",
                &elem_value.get_bytes()[..value_count]
            );
        }
        ElemInfo::Integer(info) => {
            let value_count = info.get_property_value_count() as usize;
            println!(
                "  values:         {:?}",
                &elem_value.get_int()[..value_count]
            );
        }
        ElemInfo::Integer64(info) => {
            let value_count = info.get_property_value_count() as usize;
            println!(
                "  values:         {:?}",
                &elem_value.get_int64()[..value_count]
            );
        }
        ElemInfo::Enumerated(info) => {
            let labels = info.get_property_labels();
            let value_count = info.get_property_value_count() as usize;
            let vals: Vec<_> = elem_value
                .get_enum()
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
    let card_id_list = match alsactl::functions::get_card_id_list() {
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
                eprintln!("Fail to dump the data of element: {}", elem_id.get_name());
                std::process::exit(1);
            }
        });
    });
}
