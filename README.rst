====================
alsa-gobject Rust bindings
====================

2022/03/17
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for libraries in alsa-gobject project.

  * `<https://github.com/alsa-project/alsa-gobject/>`_

License
=======

MIT License

Dependencies
============

* Rust version 1.57 or later (edition 2021)
* `alsa-gobject <https://github.com/alsa-project/alsa-gobject/>`_
* FFI crates (``alsactl-sys``, ``alsahwdep-sys``, ``alsarawmidi-sys``, ``alsatimer-sys``)

  * ``libc`` >= 0.2
  * ``glib-sys`` >= 0.15
  * ``gobject-sys`` >= 0.15

* API crates (``alsactl``, ``alsahwdep``, ``alsarawmidi``, ``alsatimer``)

  * ``libc`` >= 0.2
  * ``bitflags`` >= 1.0
  * ``glib`` >= 0.15
  * FFI crate per each (``alsactl-sys``, ``alsahwdep-sys``, ``alsarawmidi-sys``, ``alsatimer-sys``)

Sample code
===========

Dump the data of control element which belongs to sound card::

    use alsactl::{Card, CardExt, CardExtManual};
    use alsactl::{ElemInfoExt, ElemInfoExtManual};
    use alsactl::ElemType;
    use alsactl::ElemAccessFlag;
    use alsactl::{ElemValue, ElemValueExtManual};
    
    use glib::Error;
    
    fn dump_elem_data(card: &alsactl::Card, elem_id: &alsactl::ElemId) -> Result<(), Error> {
        println!("Element {}:", elem_id.get_numid());
        println!("  device_id:      {}", elem_id.get_device_id());
        println!("  subdevice_id:   {}", elem_id.get_subdevice_id());
        println!("  name :          {}", elem_id.get_name());
        println!("  iface:          {}", elem_id.get_iface());
        println!("  index:          {}", elem_id.get_index());
    
        let elem_info = card.get_elem_info(elem_id)?;
    
        let elem_access = elem_info.get_property_access();
        let elem_type = elem_info.get_property_type();
        let value_count = elem_info.get_property_value_count() as usize;
        println!("  access:         {:?}", elem_access);
        println!("  owner:          {}", elem_info.get_property_owner());
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
                let data = elem_info.get_enum_data()?;
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
                let mut vals = vec![false;value_count];
                elem_value.get_bool(&mut vals);
                println!("  values:         {:?}", vals);
            }
            ElemType::Integer => {
                let mut vals = vec![0;value_count];
                elem_value.get_int(&mut vals);
                println!("  values:         {:?}", vals);
            }
            ElemType::Enumerated => {
                let mut vals = vec![0;value_count];
                elem_value.get_enum(&mut vals);
                println!("  values:         {:?}", vals);
            }
            ElemType::Bytes => {
                let mut vals = vec![0;value_count];
                elem_value.get_bytes(&mut vals);
                println!("  values:         {:?}", vals);
            }
            ElemType::Iec60958 => {
                let mut channel_status = vec![0;24];
                let mut user_data = vec![0;147];
                elem_value.get_iec60958_channel_status(&mut channel_status);
                elem_value.get_iec60958_user_data(&mut user_data);
                println!("  channel_status: {:?}", channel_status);
                println!("  user_data:      {:?}", user_data);
            }
            ElemType::Integer64 => {
                let mut vals = vec![0;value_count];
                elem_value.get_int64(&mut vals);
                println!("  values:         {:?}", vals);
            }
            _ => {
            },
        }
    
        if elem_access.contains(ElemAccessFlag::TLV_READ) {
            let mut cntr = vec![0;64];
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
                    eprintln!("Fail to dump the data of element: {}",
                              elem_id.get_name());
                    std::process::exit(1);
                }
            });
        });
    }

Dump the note/ctl data of event received by user client of ALSA Sequencer::

  use alsaseq::{UserClient, UserClientExt, UserClientExtManual};
  use alsaseq::{ClientInfo, ClientInfoExt};
  use alsaseq::{PortInfo, PortInfoExt};
  use alsaseq::{EventType, EventTimestampMode, EventTimeMode, EventPriorityMode};
  use alsaseq::{SpecificClientId, SpecificPortId, SpecificQueueId};
  use alsaseq::{PortCapFlag, PortAttrFlag};
  use alsaseq::{QueueInfo, QueueInfoExt};
  use alsaseq::{EventCntr, EventCntrExt, EventCntrExtManual};
  use alsaseq::Addr;
  
  use glib::Error;
  use glib::{MainLoop, source};
  use glib::translate::ToGlib;
  
  use nix::sys::signal;
  
  use std::sync::Arc;
  
  fn prepare_client(name: &str) -> Result<(UserClient, ClientInfo), Error> {
      let client = UserClient::new();
      if client.open(0).is_err() {
          eprintln!("Fail to open ALSA Sequencer character device.");
          std::process::exit(1);
      }
  
      let mut info = ClientInfo::new();
      if client.get_info(&mut info).is_err() {
          eprintln!("Fail to get the information of client.");
          std::process::exit(1);
      }
  
      info.set_property_name(Some(name));
      if client.set_info(&mut info).is_err() {
          eprintln!("Fail to set the information of clinent.");
          std::process::exit(1);
      }
  
      Ok((client, info))
  }
  
  fn prepare_port(client: &UserClient, name: &str) -> Result<PortInfo, Error> {
      let mut info = PortInfo::new();
  
      info.set_property_name(Some(name));
  
      let caps = PortCapFlag::READ |
                 PortCapFlag::WRITE |
                 PortCapFlag::DUPLEX |
                 PortCapFlag::SUBS_READ |
                 PortCapFlag::SUBS_WRITE;
      info.set_property_caps(caps);
  
      let attrs = PortAttrFlag::MIDI_GENERIC |
                  PortAttrFlag::SOFTWARE |
                  PortAttrFlag::APPLICATION;
      info.set_property_attrs(attrs);
  
      client.create_port(&mut info)?;
  
      Ok(info)
  }
  
  fn prepare_queue(client: &UserClient, port: &PortInfo, name: &str) -> Result<QueueInfo, Error> {
      let mut info = QueueInfo::new();
  
      info.set_property_name(Some(name));
      info.set_property_locked(true);
  
      client.create_queue(&mut info)?;
  
      let ev_cntr = EventCntr::new(1)?;
      ev_cntr.set_event_type(0, EventType::Start)?;
      ev_cntr.set_tstamp_mode(0, EventTimestampMode::Real)?;
      ev_cntr.set_time_mode(0, EventTimeMode::Rel)?;
      ev_cntr.set_priority_mode(0, EventPriorityMode::Normal)?;
      ev_cntr.set_tag(0, 0)?;
      ev_cntr.set_queue_id(0, SpecificQueueId::Direct.to_glib() as u8)?;
      let addr = Addr::new(SpecificClientId::System.to_glib() as u8, SpecificPortId::Timer.to_glib() as u8);
      ev_cntr.set_dst(0, &addr)?;
      if let Some(addr) = port.get_property_addr() {
          ev_cntr.set_src(0, &addr)?;
      }
      let mut data = ev_cntr.get_queue_data(0)?;
      data.set_queue_id(info.get_property_queue_id() as u8);
      ev_cntr.set_queue_data(0, &data)?;
  
      client.schedule_event(&ev_cntr, 1)?;
  
      Ok(info)
  }
  
  fn dump_info(client: &ClientInfo, port: &PortInfo, queue: &QueueInfo) {
      println!("Client: {}", client.get_property_name().expect(""));
      println!("  card-id:                {}", client.get_property_card_id());
      println!("  client-id:              {}", client.get_property_client_id());
      println!("  filter-attrs:           {:?}", client.get_property_filter_attributes());
      println!("  lost-count:             {}", client.get_property_lost_count());
      println!("  port-count:             {}", client.get_property_port_count());
      println!("  process-id:             {}", client.get_property_process_id());
      println!("  type:                   {}", client.get_property_type());
      println!("  use-filter:             {}", client.get_property_use_filter());
  
      println!("Port: {}", port.get_property_name().expect(""));
      if let Some(addr) = port.get_property_addr() {
          println!("  client:                 {}", addr.get_client_id());
          println!("  port:                   {}", addr.get_port_id());
      }
      println!("  attrs:                  {:?}", port.get_property_attrs());
      println!("  caps:                   {:?}", port.get_property_caps());
      println!("  midi channels:          {}", port.get_property_midi_channels());
      println!("  midi voices:            {}", port.get_property_midi_voices());
      println!("  queue-id:               {}", port.get_property_queue_id());
      println!("  read users:             {}", port.get_property_read_users());
      println!("  synth voices:           {}", port.get_property_synth_voices());
      println!("  timestamp-mode:         {}", port.get_property_timestamp_mode());
      println!("  timestamp-overwrite:    {}", port.get_property_timestamp_overwrite());
      println!("  write users:            {}", port.get_property_write_users());
  
      println!("Queue: {}", queue.get_property_name().expect(""));
      println!("  client-id:              {}", queue.get_property_client_id());
      println!("  locked:                 {}", queue.get_property_locked());
      println!("  queue-id:               {}", queue.get_property_queue_id());
  }
  
  fn run_dispatcher(client: &UserClient) -> Result<(), Error> {
      let dispatcher = MainLoop::new(None, false);
      let ctx = dispatcher.get_context();
  
      let dispatcher_cntr = Arc::new(dispatcher);
      let d = dispatcher_cntr.clone();
  
      let src = source::unix_signal_source_new(signal::Signal::SIGINT as i32, None,
                                               source::PRIORITY_DEFAULT_IDLE, move || {
          d.quit();
          source::Continue(true)
      });
      src.attach(Some(&ctx));
  
      let src = client.create_source()?;
      src.attach(Some(&ctx));
  
      client.connect_handle_event(|_, ev_cntr| {
          let count = ev_cntr.count_events();
          println!("Event count: {}", count);
          (0..count).try_for_each(|i| {
              let ev_type = ev_cntr.get_event_type(i)?;
              let tstamp_mode = ev_cntr.get_tstamp_mode(i)?;
              println!("  Event {}:           {}", i, ev_type);
              println!("    length-mode:      {}", ev_cntr.get_length_mode(i)?);
              println!("    priority-mode:    {}", ev_cntr.get_priority_mode(i)?);
              println!("    time-mode:        {}", ev_cntr.get_time_mode(i)?);
              println!("    tstamp-mode:      {}", tstamp_mode);
              println!("    queue-id:         {}", ev_cntr.get_queue_id(i)?);
              println!("    tag:              {}", ev_cntr.get_tag(i)?);
  
              let mut tstamp = ev_cntr.get_tstamp(i)?;
              if tstamp_mode == EventTimestampMode::Tick {
                  println!("    tick-time:        {}", tstamp.get_tick_time());
              } else {
                  let real_time = tstamp.get_real_time();
                  println!("    real-time:        {}.{}", real_time[0], real_time[1]);
              }
  
              let src = ev_cntr.get_src(i)?;
              println!("    src:");
              println!("      client-id:      {}", src.get_client_id());
              println!("      port-id:        {}", src.get_port_id());
  
              let dst = ev_cntr.get_dst(i)?;
              println!("    dst:");
              println!("      client-id:      {}", dst.get_client_id());
              println!("      port-id:        {}", dst.get_port_id());
  
              match ev_type {
                  EventType::Note | EventType::Noteon | EventType::Noteoff | EventType::Keypress => {
                      let data = ev_cntr.get_note_data(i)?;
                      println!("    note data:");
                      println!("      channel:        {}", data.get_channel());
                      println!("      note:           {}", data.get_note());
                      println!("      duration:       {}", data.get_duration());
                      println!("      velocity:       {}", data.get_velocity());
                      println!("      off-velocity:   {}", data.get_off_velocity());
                  },
                  EventType::Pgmchange | EventType::Chanpress | EventType::Pitchbend | EventType::Control14 |
                  EventType::Nonregparam | EventType::Regparam | EventType::Songpos | EventType::Songsel |
                  EventType::Qframe | EventType::Timesign | EventType::Keysign => {
                      let data = ev_cntr.get_ctl_data(i)?;
                      println!("    ctl data:");
                      println!("      channel:        {}", data.get_channel());
                      println!("      param:          {}", data.get_param());
                      println!("      value:          {}", data.get_value());
                  }
                  _ => ()
              }
  
              Ok::<(), Error>(())
          }).unwrap();
      });
  
      dispatcher_cntr.run();
  
      Ok(())
  }
  
  fn main() {
      match prepare_client("focal") {
          Err(_) => eprintln!("Fail to prepare user client."),
          Ok((client, client_info)) => {
              match prepare_port(&client, "fossa") {
                  Err(_) => eprintln!("Fail to prepare port for the user client."),
                  Ok(port_info) => {
                      match prepare_queue(&client, &port_info, "20.04") {
                          Err(_) => eprintln!("Fail to prepare port for the user client."),
                          Ok(queue_info) => {
                              dump_info(&client_info, &port_info, &queue_info);
  
                              run_dispatcher(&client).unwrap();
  
                              let queue_id = queue_info.get_property_queue_id();
                              client.delete_queue(queue_id).unwrap();
                          },
                      }
                      if let Some(addr) = port_info.get_property_addr() {
                          let port_id = addr.get_port_id();
                          client.delete_port(port_id).unwrap();
                      }
                  }
              }
          }
      }
  
      std::process::exit(0);
  }

How to generate FFI and API crates
==================================

::

    $ ./generator.py

end
