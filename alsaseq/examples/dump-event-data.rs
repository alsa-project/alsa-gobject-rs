// SPDX-License-Identifier: MIT
use alsaseq::{prelude::*, *};
use glib::{source, translate::*, ControlFlow, Error, MainLoop};
use nix::sys::signal;
use std::sync::Arc;

fn prepare_client(name: &str) -> Result<(UserClient, ClientInfo), Error> {
    let client = UserClient::new();
    if client.open(0).is_err() {
        eprintln!("Fail to open ALSA Sequencer character device.");
        std::process::exit(1);
    }

    let mut info = ClientInfo::new();
    if client.info(&mut info).is_err() {
        eprintln!("Fail to get the information of client.");
        std::process::exit(1);
    }

    info.set_name(Some(name));
    if client.set_info(&mut info).is_err() {
        eprintln!("Fail to set the information of clinent.");
        std::process::exit(1);
    }

    Ok((client, info))
}

fn prepare_port(client: &UserClient, name: &str) -> Result<PortInfo, Error> {
    let mut info = PortInfo::new();

    info.set_name(Some(name));

    let caps = PortCapFlag::READ
        | PortCapFlag::WRITE
        | PortCapFlag::DUPLEX
        | PortCapFlag::SUBS_READ
        | PortCapFlag::SUBS_WRITE;
    info.set_caps(caps);

    let attrs = PortAttrFlag::MIDI_GENERIC | PortAttrFlag::SOFTWARE | PortAttrFlag::APPLICATION;
    info.set_attrs(attrs);

    client.create_port(&mut info)?;

    Ok(info)
}

fn prepare_queue(client: &UserClient, port: &PortInfo, name: &str) -> Result<QueueInfo, Error> {
    let mut info = QueueInfo::new();

    info.set_name(Some(name));
    info.set_locked(true);

    client.create_queue(&mut info)?;

    let mut ev = Event::new(EventType::Start);
    ev.set_time_mode(EventTimeMode::Rel);
    ev.set_priority_mode(EventPriorityMode::Normal);
    ev.set_tag(0);
    ev.set_queue_id(SpecificQueueId::Direct.into_glib() as u8);
    let addr = Addr::new(
        SpecificClientId::System.into_glib() as u8,
        SpecificPortId::Timer.into_glib() as u8,
    );
    ev.set_destination(&addr);
    if let Some(addr) = port.addr() {
        ev.set_source(&addr);
    }
    let mut data = ev.queue_data().unwrap();
    data.set_queue_id(info.queue_id().into());
    let _ = ev.set_queue_data(&data);

    client.schedule_event(&ev)?;

    Ok(info)
}

fn client_type_to_str(client_type: &ClientType) -> &str {
    match client_type {
        ClientType::None => "None",
        ClientType::User => "User",
        ClientType::Kernel => "Kernel",
        _ => "Unknown",
    }
}

fn event_tstamp_mode_to_str(event_tstamp_mode: &EventTstampMode) -> &str {
    match event_tstamp_mode {
        EventTstampMode::Tick => "Tick",
        EventTstampMode::Real => "Real",
        _ => "Unknown",
    }
}

fn event_type_to_str(event_type: &EventType) -> &str {
    match event_type {
        EventType::System => "System",
        EventType::Result => "Result",
        EventType::Note => "Note",
        EventType::Noteon => "Noteon",
        EventType::Noteoff => "Noteoff",
        EventType::Keypress => "Keypress",
        EventType::Controller => "Controller",
        EventType::Pgmchange => "Pgmchange",
        EventType::Chanpress => "Chanpress",
        EventType::Pitchbend => "Pitchbend",
        EventType::Control14 => "Control14",
        EventType::Nonregparam => "Nonregparam",
        EventType::Regparam => "Regparam",
        EventType::Songpos => "Songpos",
        EventType::Songsel => "Songsel",
        EventType::Qframe => "Qframe",
        EventType::Timesign => "Timesign",
        EventType::Keysign => "Keysign",
        EventType::Start => "Start",
        EventType::Continue => "Continue",
        EventType::Stop => "Stop",
        EventType::SetposTick => "SetposTick",
        EventType::SetposTime => "SetposTime",
        EventType::Tempo => "Tempo",
        EventType::Clock => "Clock",
        EventType::Tick => "Tick",
        EventType::QueueSkew => "QueueSkew",
        EventType::TuneRequest => "TuneRequest",
        EventType::Reset => "Reset",
        EventType::Sensing => "Sensing",
        EventType::Echo => "Echo",
        EventType::Oss => "Oss",
        EventType::ClientStart => "ClientStart",
        EventType::ClientExit => "ClientExit",
        EventType::ClientChange => "ClientChange",
        EventType::PortStart => "PortStart",
        EventType::PortExit => "PortExit",
        EventType::PortChange => "PortChange",
        EventType::PortSubscribed => "PortSubscribed",
        EventType::PortUnsubscribed => "PortUnsubscribed",
        EventType::Usr0 => "Usr0",
        EventType::Usr1 => "Usr1",
        EventType::Usr2 => "Usr2",
        EventType::Usr3 => "Usr3",
        EventType::Usr4 => "Usr4",
        EventType::Usr5 => "Usr5",
        EventType::Usr6 => "Usr6",
        EventType::Usr7 => "Usr7",
        EventType::Usr8 => "Usr8",
        EventType::Usr9 => "Usr9",
        EventType::Sysex => "Sysex",
        EventType::Bounce => "Bounce",
        EventType::UsrVar0 => "UsrVar0",
        EventType::UsrVar1 => "UsrVar1",
        EventType::UsrVar2 => "UsrVar2",
        EventType::UsrVar3 => "UsrVar3",
        EventType::UsrVar4 => "UsrVar4",
        EventType::None => "None",
        _ => "Unknown",
    }
}

fn event_length_mode_to_str(event_length_mode: &EventLengthMode) -> &str {
    match event_length_mode {
        EventLengthMode::Fixed => "Fixed",
        EventLengthMode::Variable => "Variable",
        EventLengthMode::Pointer => "Pointer",
        _ => "Unknown",
    }
}

fn dump_info(client: &ClientInfo, port: &PortInfo, queue: &QueueInfo) {
    println!("Client: {}", client.name().expect(""));
    println!("  card-id:                {}", client.card_id());
    println!("  client-id:              {}", client.client_id());
    println!("  filter-attrs:           {:?}", client.filter_attributes());
    println!("  lost-count:             {}", client.lost_count());
    println!("  port-count:             {}", client.port_count());
    println!("  process-id:             {}", client.process_id());
    println!(
        "  type:                   {}",
        client_type_to_str(&client.type_())
    );
    println!("  use-filter:             {}", client.uses_filter());

    println!("Port: {}", port.name().expect(""));
    if let Some(addr) = port.addr() {
        println!("  client:                 {}", addr.client_id());
        println!("  port:                   {}", addr.port_id());
    }
    println!("  attrs:                  {:?}", port.attrs());
    println!("  caps:                   {:?}", port.caps());
    println!("  midi channels:          {}", port.midi_channels());
    println!("  midi voices:            {}", port.midi_voices());
    println!("  queue-id:               {}", port.queue_id());
    println!("  read users:             {}", port.read_users());
    println!("  synth voices:           {}", port.synth_voices());
    println!(
        "  tstamp-mode:         {}",
        event_tstamp_mode_to_str(&port.tstamp_mode())
    );
    println!("  tstamp-overwrite:    {}", port.is_tstamp_overwrite());
    println!("  write users:            {}", port.write_users());

    println!("Queue: {}", queue.name().expect(""));
    println!("  client-id:              {}", queue.client_id());
    println!("  locked:                 {}", queue.is_locked());
    println!("  queue-id:               {}", queue.queue_id());
}

fn event_priority_mode_to_str(event_priority_mode: &EventPriorityMode) -> &str {
    match event_priority_mode {
        EventPriorityMode::Normal => "Normal",
        EventPriorityMode::High => "High",
        _ => "Unknown",
    }
}

fn event_time_mode_to_str(event_time_mode: &EventTimeMode) -> &str {
    match event_time_mode {
        EventTimeMode::Abs => "Abs",
        EventTimeMode::Rel => "Rel",
        _ => "Unknown",
    }
}

fn run_dispatcher(client: &UserClient) -> Result<(), Error> {
    let dispatcher = MainLoop::new(None, false);
    let ctx = dispatcher.context();

    let dispatcher_cntr = Arc::new(dispatcher);
    let d = dispatcher_cntr.clone();

    let src = source::unix_signal_source_new(
        signal::Signal::SIGINT as i32,
        None,
        source::Priority::DEFAULT_IDLE,
        move || {
            d.quit();
            ControlFlow::Continue
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
                let ev_type = ev.event_type();
                let tstamp_mode = ev.tstamp_mode();
                println!("  Event {}:           {}", i, event_type_to_str(&ev_type));
                println!(
                    "    length-mode:      {}",
                    event_length_mode_to_str(&ev.length_mode())
                );
                println!(
                    "    priority-mode:    {}",
                    event_priority_mode_to_str(&ev.priority_mode())
                );
                println!(
                    "    time-mode:        {}",
                    event_time_mode_to_str(&ev.time_mode())
                );
                println!(
                    "    tstamp-mode:      {}",
                    event_tstamp_mode_to_str(&tstamp_mode)
                );
                println!("    queue-id:         {}", ev.queue_id());
                println!("    tag:              {}", ev.tag());

                if tstamp_mode == EventTstampMode::Tick {
                    println!("    tick-time:        {}", ev.tick_time().unwrap());
                } else {
                    let real_time = ev.real_time().unwrap();
                    println!("    real-time:        {}.{}", real_time[0], real_time[1]);
                }

                let src = ev.source();
                println!("    src:");
                println!("      client-id:      {}", src.client_id());
                println!("      port-id:        {}", src.port_id());

                let dst = ev.destination();
                println!("    dst:");
                println!("      client-id:      {}", dst.client_id());
                println!("      port-id:        {}", dst.port_id());

                // Just for events used frequently.
                match ev_type {
                    EventType::Note
                    | EventType::Noteon
                    | EventType::Noteoff
                    | EventType::Keypress => {
                        let data = ev.note_data().unwrap();
                        println!("    note data:");
                        println!("      channel:        {}", data.channel());
                        println!("      note:           {}", data.note());
                        println!("      duration:       {}", data.duration());
                        println!("      velocity:       {}", data.velocity());
                        println!("      off-velocity:   {}", data.off_velocity());
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
                        let data = ev.ctl_data().unwrap();
                        println!("    ctl data:");
                        println!("      channel:        {}", data.channel());
                        println!("      param:          {}", data.param());
                        println!("      value:          {}", data.value());
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

                        let queue_id = queue_info.queue_id();
                        client.delete_queue(queue_id).unwrap();
                    }
                }
                if let Some(addr) = port_info.addr() {
                    let port_id = addr.port_id();
                    client.delete_port(port_id).unwrap();
                }
            }
        },
    }

    std::process::exit(0);
}
