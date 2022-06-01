// SPDX-License-Identifier: MIT
extern crate alsaseq;
extern crate glib;
extern crate nix;
use alsaseq::{traits::*, *};
use glib::{source, translate::*, Error, MainLoop};
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

    let caps = PortCapFlag::READ
        | PortCapFlag::WRITE
        | PortCapFlag::DUPLEX
        | PortCapFlag::SUBS_READ
        | PortCapFlag::SUBS_WRITE;
    info.set_property_caps(caps);

    let attrs = PortAttrFlag::MIDI_GENERIC | PortAttrFlag::SOFTWARE | PortAttrFlag::APPLICATION;
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
    let addr = Addr::new(
        SpecificClientId::System.to_glib() as u8,
        SpecificPortId::Timer.to_glib() as u8,
    );
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
    println!(
        "  card-id:                {}",
        client.get_property_card_id()
    );
    println!(
        "  client-id:              {}",
        client.get_property_client_id()
    );
    println!(
        "  filter-attrs:           {:?}",
        client.get_property_filter_attributes()
    );
    println!(
        "  lost-count:             {}",
        client.get_property_lost_count()
    );
    println!(
        "  port-count:             {}",
        client.get_property_port_count()
    );
    println!(
        "  process-id:             {}",
        client.get_property_process_id()
    );
    println!("  type:                   {}", client.get_property_type());
    println!(
        "  use-filter:             {}",
        client.get_property_use_filter()
    );

    println!("Port: {}", port.get_property_name().expect(""));
    if let Some(addr) = port.get_property_addr() {
        println!("  client:                 {}", addr.get_client_id());
        println!("  port:                   {}", addr.get_port_id());
    }
    println!("  attrs:                  {:?}", port.get_property_attrs());
    println!("  caps:                   {:?}", port.get_property_caps());
    println!(
        "  midi channels:          {}",
        port.get_property_midi_channels()
    );
    println!(
        "  midi voices:            {}",
        port.get_property_midi_voices()
    );
    println!("  queue-id:               {}", port.get_property_queue_id());
    println!(
        "  read users:             {}",
        port.get_property_read_users()
    );
    println!(
        "  synth voices:           {}",
        port.get_property_synth_voices()
    );
    println!(
        "  timestamp-mode:         {}",
        port.get_property_timestamp_mode()
    );
    println!(
        "  timestamp-overwrite:    {}",
        port.get_property_timestamp_overwrite()
    );
    println!(
        "  write users:            {}",
        port.get_property_write_users()
    );

    println!("Queue: {}", queue.get_property_name().expect(""));
    println!(
        "  client-id:              {}",
        queue.get_property_client_id()
    );
    println!("  locked:                 {}", queue.get_property_locked());
    println!(
        "  queue-id:               {}",
        queue.get_property_queue_id()
    );
}

fn run_dispatcher(client: &UserClient) -> Result<(), Error> {
    let dispatcher = MainLoop::new(None, false);
    let ctx = dispatcher.get_context();

    let dispatcher_cntr = Arc::new(dispatcher);
    let d = dispatcher_cntr.clone();

    let src = source::unix_signal_source_new(
        signal::Signal::SIGINT as i32,
        None,
        source::PRIORITY_DEFAULT_IDLE,
        move || {
            d.quit();
            source::Continue(true)
        },
    );
    src.attach(Some(&ctx));

    let src = client.create_source()?;
    src.attach(Some(&ctx));

    client.connect_handle_event(|_, ev_cntr| {
        let count = ev_cntr.count_events();
        println!("Event count: {}", count);
        (0..count)
            .try_for_each(|i| {
                let ev_type = ev_cntr.get_event_type(i)?;
                let tstamp_mode = ev_cntr.get_tstamp_mode(i)?;
                println!("  Event {}:           {}", i, ev_type);
                println!("    length-mode:      {}", ev_cntr.get_length_mode(i)?);
                println!("    priority-mode:    {}", ev_cntr.get_priority_mode(i)?);
                println!("    time-mode:        {}", ev_cntr.get_time_mode(i)?);
                println!("    tstamp-mode:      {}", tstamp_mode);
                println!("    queue-id:         {}", ev_cntr.get_queue_id(i)?);
                println!("    tag:              {}", ev_cntr.get_tag(i)?);

                let tstamp = ev_cntr.get_tstamp(i)?;
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
                    EventType::Note
                    | EventType::Noteon
                    | EventType::Noteoff
                    | EventType::Keypress => {
                        let data = ev_cntr.get_note_data(i)?;
                        println!("    note data:");
                        println!("      channel:        {}", data.get_channel());
                        println!("      note:           {}", data.get_note());
                        println!("      duration:       {}", data.get_duration());
                        println!("      velocity:       {}", data.get_velocity());
                        println!("      off-velocity:   {}", data.get_off_velocity());
                    }
                    EventType::Pgmchange
                    | EventType::Chanpress
                    | EventType::Pitchbend
                    | EventType::Control14
                    | EventType::Nonregparam
                    | EventType::Regparam
                    | EventType::Songpos
                    | EventType::Songsel
                    | EventType::Qframe
                    | EventType::Timesign
                    | EventType::Keysign => {
                        let data = ev_cntr.get_ctl_data(i)?;
                        println!("    ctl data:");
                        println!("      channel:        {}", data.get_channel());
                        println!("      param:          {}", data.get_param());
                        println!("      value:          {}", data.get_value());
                    }
                    _ => (),
                }

                Ok::<(), Error>(())
            })
            .unwrap();
    });

    dispatcher_cntr.run();

    Ok(())
}

fn main() {
    match prepare_client("focal") {
        Err(_) => eprintln!("Fail to prepare user client."),
        Ok((client, client_info)) => match prepare_port(&client, "fossa") {
            Err(_) => eprintln!("Fail to prepare port for the user client."),
            Ok(port_info) => {
                match prepare_queue(&client, &port_info, "20.04") {
                    Err(_) => eprintln!("Fail to prepare port for the user client."),
                    Ok(queue_info) => {
                        dump_info(&client_info, &port_info, &queue_info);

                        run_dispatcher(&client).unwrap();

                        let queue_id = queue_info.get_property_queue_id();
                        client.delete_queue(queue_id).unwrap();
                    }
                }
                if let Some(addr) = port_info.get_property_addr() {
                    let port_id = addr.get_port_id();
                    client.delete_port(port_id).unwrap();
                }
            }
        },
    }

    std::process::exit(0);
}
