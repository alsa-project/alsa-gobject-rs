// SPDX-License-Identifier: MIT
extern crate alsaseq;
extern crate glib;
extern crate nix;
use alsaseq::{prelude::*, *};
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

    let mut ev = Event::new(EventType::Start);
    ev.set_time_mode(EventTimeMode::Rel);
    ev.set_priority_mode(EventPriorityMode::Normal);
    ev.set_tag(0);
    ev.set_queue_id(SpecificQueueId::Direct.to_glib() as u8);
    let addr = Addr::new(
        SpecificClientId::System.to_glib() as u8,
        SpecificPortId::Timer.to_glib() as u8,
    );
    ev.set_destination(&addr);
    if let Some(addr) = port.get_property_addr() {
        ev.set_source(&addr);
    }
    let mut data = ev.get_queue_data().unwrap();
    data.set_queue_id(info.get_property_queue_id().into());
    let _ = ev.set_queue_data(&data);

    client.schedule_event(&ev)?;

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
    println!("  tstamp-mode:         {}", port.get_property_tstamp_mode());
    println!(
        "  tstamp-overwrite:    {}",
        port.get_property_tstamp_overwrite()
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
        let events = ev_cntr.deserialize();
        println!("Event count: {}", events.len());
        events
            .iter()
            .enumerate()
            .try_for_each(|(i, ev)| {
                let ev_type = ev.get_event_type();
                let tstamp_mode = ev.get_tstamp_mode();
                println!("  Event {}:           {}", i, ev_type);
                println!("    length-mode:      {}", ev.get_length_mode());
                println!("    priority-mode:    {}", ev.get_priority_mode());
                println!("    time-mode:        {}", ev.get_time_mode());
                println!("    tstamp-mode:      {}", tstamp_mode);
                println!("    queue-id:         {}", ev.get_queue_id());
                println!("    tag:              {}", ev.get_tag());

                if tstamp_mode == EventTstampMode::Tick {
                    println!("    tick-time:        {}", ev.get_tick_time().unwrap());
                } else {
                    let real_time = ev.get_real_time().unwrap();
                    println!("    real-time:        {}.{}", real_time[0], real_time[1]);
                }

                let src = ev.get_source();
                println!("    src:");
                println!("      client-id:      {}", src.get_client_id());
                println!("      port-id:        {}", src.get_port_id());

                let dst = ev.get_destination();
                println!("    dst:");
                println!("      client-id:      {}", dst.get_client_id());
                println!("      port-id:        {}", dst.get_port_id());

                // Just for events used frequently.
                match ev_type {
                    EventType::Note
                    | EventType::Noteon
                    | EventType::Noteoff
                    | EventType::Keypress => {
                        let data = ev.get_note_data().unwrap();
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
                        let data = ev.get_ctl_data().unwrap();
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
